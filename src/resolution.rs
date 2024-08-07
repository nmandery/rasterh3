use geo::{AffineOps, AffineTransform};
use geo_types::Rect;
use h3o::{LatLng, Resolution};

use crate::{error::Error, sphere::AreaOnSphere, AxisOrder};

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
        let bbox_array = Rect::new(
            (0.0_f64, 0.0_f64),
            (
                (shape[axis_order.x_axis()] - 1) as f64,
                (shape[axis_order.y_axis()] - 1) as f64,
            ),
        )
        .affine_transform(transform);
        let area_pixel = bbox_array.area_on_sphere_m2()
            / (shape[axis_order.x_axis()] * shape[axis_order.y_axis()]) as f64;
        let center_of_array: LatLng = bbox_array.center().try_into()?;

        let mut nearest_h3_res = Resolution::Zero;
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
                    if let Some(old_area_difference) = area_difference {
                        if old_area_difference < new_area_difference {
                            nearest_h3_res = h3_res.pred().unwrap_or(Resolution::Zero);
                            break;
                        } else {
                            area_difference = Some(new_area_difference);
                        }
                    } else {
                        area_difference = Some(new_area_difference);
                    }
                }
            }
        }

        Ok(nearest_h3_res)
    }
}

#[cfg(test)]
mod tests {
    use h3o::Resolution;

    use crate::resolution::ResolutionSearchMode;
    use crate::AxisOrder;

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
}
