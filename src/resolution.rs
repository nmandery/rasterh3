use geo::{AffineOps, AffineTransform};
use geo_types::Rect;
use h3o::{LatLng, Resolution};

use crate::{AxisOrder, error::Error, sphere::AreaOnSphere};

#[derive(Copy, Clone)]
pub enum ResolutionSearchMode {
    /// Chose the H3 resolution where the difference in the area of a pixel and the h3index is
    /// as small as possible.
    MinDiff,

    /// Chose the H3 resolution where the area of the h3index is smaller than the area of a pixel.
    SmallerThanPixel,
}

impl ResolutionSearchMode {
    /// Find the H3 resolution closed to the size of a pixel in an array
    /// of the given shape with the given transform.
    pub fn nearest_h3_resolution(
        &self,
        shape: [usize; 2],
        transform: &AffineTransform<f64>,
        axis_order: &AxisOrder,
    ) -> Result<Resolution, Error> {
        if shape[0] == 0 || shape[1] == 0 {
            return Err(Error::EmptyArray);
        }
        let x_size = shape[axis_order.x_axis()];
        let y_size = shape[axis_order.y_axis()];
        // Use the full pixel footprint (outer corners of the first and last
        // pixels) rather than `(shape - 1)` so the per-pixel area is correct,
        // including for small and 1x1 arrays.
        let bbox_array = Rect::new((0.0_f64, 0.0_f64), (x_size as f64, y_size as f64))
            .affine_transform(transform);
        let area_pixel = bbox_array.area_on_sphere_m2() / (x_size * y_size) as f64;
        let center_of_array: LatLng = bbox_array.center().try_into()?;

        // Default to the finest resolution. This is the correct result for
        // `SmallerThanPixel` when no H3 cell is smaller than the pixel, and is
        // overwritten by every iteration of the `MinDiff` arm below.
        let mut nearest_h3_res = Resolution::Fifteen;
        let mut area_difference = None;
        for h3_res in Resolution::range(Resolution::Zero, Resolution::Fifteen) {
            let area_h3_index = center_of_array.to_cell(h3_res).area_m2();

            match self {
                Self::SmallerThanPixel => {
                    if area_h3_index <= area_pixel {
                        nearest_h3_res = h3_res;
                        break;
                    }
                }

                Self::MinDiff => {
                    let new_area_difference = if area_h3_index > area_pixel {
                        area_h3_index - area_pixel
                    } else {
                        area_pixel - area_h3_index
                    };
                    // H3 cell areas decrease monotonically with increasing
                    // resolution, so the absolute difference to the pixel area
                    // has a single minimum. Track it explicitly and stop once it
                    // starts growing again.
                    if let Some(old_area_difference) = area_difference
                        && old_area_difference < new_area_difference
                    {
                        // `nearest_h3_res` already holds the previous (minimal) resolution.
                        break;
                    }
                    area_difference = Some(new_area_difference);
                    nearest_h3_res = h3_res;
                }
            }
        }

        Ok(nearest_h3_res)
    }
}

#[cfg(test)]
mod tests {
    use h3o::Resolution;

    use crate::AxisOrder;
    use crate::resolution::ResolutionSearchMode;

    #[test]
    fn test_nearest_h3_resolution() {
        // transform of the included r.tiff
        let gt = crate::transform::from_rasterio(&[
            0.0011965049999999992,
            0.0,
            8.11377,
            0.0,
            -0.001215135,
            49.40792,
        ]);
        let h3_res1 = ResolutionSearchMode::MinDiff
            .nearest_h3_resolution([2000_usize, 2000_usize], &gt, &AxisOrder::YX)
            .unwrap();
        assert_eq!(h3_res1, Resolution::Ten); // TODO: validate

        let h3_res2 = ResolutionSearchMode::SmallerThanPixel
            .nearest_h3_resolution([2000_usize, 2000_usize], &gt, &AxisOrder::YX)
            .unwrap();
        assert_eq!(h3_res2, Resolution::Eleven); // TODO: validate
    }

    #[test]
    fn test_nearest_h3_resolution_pixel_smaller_than_fines_cell() {
        // pixel much smaller than the smallest H3 cell (res 15)
        let gt = crate::transform::from_gdal(&[10.0, 0.0000001, 0.0, 50.0, 0.0, -0.0000001]);
        let h3_res_min_diff = ResolutionSearchMode::MinDiff
            .nearest_h3_resolution([100_usize, 100_usize], &gt, &AxisOrder::YX)
            .unwrap();
        // the closest resolution is the finest one (differences decrease
        // monotonically up to res 15); the old implementation returned res 0 here.
        assert_eq!(h3_res_min_diff, Resolution::Fifteen);

        let h3_res_smaller = ResolutionSearchMode::SmallerThanPixel
            .nearest_h3_resolution([100_usize, 100_usize], &gt, &AxisOrder::YX)
            .unwrap();
        // no H3 cell is smaller than the pixel; fall back to the finest resolution
        // instead of returning res 0 as the old implementation did.
        assert_eq!(h3_res_smaller, Resolution::Fifteen);
    }

    #[test]
    fn test_nearest_h3_resolution_single_pixel_array() {
        // a 1x1 array: with the old `(shape - 1)` bbox the footprint was a point
        // and the per-pixel area was estimated as 0, yielding res 0. The full
        // footprint must give a real per-pixel area and a sane resolution.
        let gt = crate::transform::from_gdal(&[0.0, 1.0, 0.0, 0.0, 0.0, -1.0]);
        let h3_res = ResolutionSearchMode::MinDiff
            .nearest_h3_resolution([1_usize, 1_usize], &gt, &AxisOrder::YX)
            .unwrap();
        // a 1x1 degree pixel is far smaller than a res 0 cell (~4.4e12 m^2), so
        // the result must not be res 0.
        assert_ne!(h3_res, Resolution::Zero);
    }
}
