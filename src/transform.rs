use geo::AffineTransform;

/// Construct from a f64 array in the ordering used by [GDAL](https://gdal.org/).
pub fn from_gdal(t: &[f64; 6]) -> AffineTransform<f64> {
    AffineTransform::new(t[1], t[2], t[0], t[4], t[5], t[3])
}

/// Construct from a f64 array in the ordering used by [rasterio](https://github.com/rasterio/rasterio/).
pub fn from_rasterio(transform: &[f64; 6]) -> AffineTransform<f64> {
    AffineTransform::new(
        transform[0],
        transform[1],
        transform[2],
        transform[3],
        transform[4],
        transform[5],
    )
}

#[cfg(test)]
mod tests {
    /*
    $ gdalinfo data/r.tiff
    Driver: GTiff/GeoTIFF
    Files: data/r.tiff
    Size is 2000, 2000
    Coordinate System is:
    GEOGCRS["WGS 84",
        DATUM["World Geodetic System 1984",
            ELLIPSOID["WGS 84",6378137,298.257223563,
                LENGTHUNIT["metre",1]]],
        PRIMEM["Greenwich",0,
            ANGLEUNIT["degree",0.0174532925199433]],
        CS[ellipsoidal,2],
            AXIS["geodetic latitude (Lat)",north,
                ORDER[1],
                ANGLEUNIT["degree",0.0174532925199433]],
            AXIS["geodetic longitude (Lon)",east,
                ORDER[2],
                ANGLEUNIT["degree",0.0174532925199433]],
        ID["EPSG",4326]]
    Data axis to CRS axis mapping: 2,1
    Origin = (8.113770000000001,49.407919999999997)
    Pixel Size = (0.001196505000000,-0.001215135000000)
    Metadata:
      AREA_OR_POINT=Area
    Image Structure Metadata:
      COMPRESSION=LZW
      INTERLEAVE=BAND
    Corner Coordinates:
    Upper Left  (   8.1137700,  49.4079200) (  8d 6'49.57"E, 49d24'28.51"N)
    Lower Left  (   8.1137700,  46.9776500) (  8d 6'49.57"E, 46d58'39.54"N)
    Upper Right (  10.5067800,  49.4079200) ( 10d30'24.41"E, 49d24'28.51"N)
    Lower Right (  10.5067800,  46.9776500) ( 10d30'24.41"E, 46d58'39.54"N)
    Center      (   9.3102750,  48.1927850) (  9d18'36.99"E, 48d11'34.03"N)
    Band 1 Block=2000x4 Type=Byte, ColorInterp=Gray
      NoData Value=0
     */

    use approx::assert_relative_eq;
    use geo::{AffineOps, AffineTransform};
    use geo_types::point;

    use crate::transform::{from_gdal, from_rasterio};

    fn r_tiff_test_helper(gt: &AffineTransform<f64>) {
        // upper left pixel
        let px_ul = point! { x: 0., y: 0. };

        let coord_ul = px_ul.affine_transform(gt);
        assert_relative_eq!(coord_ul.x(), 8.11377);
        assert_relative_eq!(coord_ul.y(), 49.40792);

        let gt_inv = gt.inverse().unwrap();
        let px_ul_back = coord_ul.affine_transform(&gt_inv);
        assert_relative_eq!(px_ul_back.x(), 0.0);
        assert_relative_eq!(px_ul_back.y(), 0.0);
    }

    #[test]
    fn test_r_tiff_from_gdal() {
        /*
        Python 3.8.5 (default, Jul 28 2020, 12:59:40)
        [GCC 9.3.0] on linux
        >>> from osgeo import gdal
        >>> ds = gdal.Open("data/r.tiff")
        >>> ds.GetGeoTransform()
        (8.11377, 0.0011965049999999992, 0.0, 49.40792, 0.0, -0.001215135)
         */
        let gt = from_gdal(&[
            8.11377,
            0.0011965049999999992,
            0.0,
            49.40792,
            0.0,
            -0.001215135,
        ]);
        r_tiff_test_helper(&gt);
    }

    #[test]
    fn test_r_tiff_from_rasterio() {
        /*
        Python 3.8.5 (default, Jul 28 2020, 12:59:40)
        [GCC 9.3.0] on linux
         >>> import rasterio
        >>> ds = rasterio.open("data/r.tiff")
        >>> ds.transform
        Affine(0.0011965049999999992, 0.0, 8.11377,
               0.0, -0.001215135, 49.40792)
         */
        let gt = from_rasterio(&[
            0.0011965049999999992,
            0.0,
            8.11377,
            0.0,
            -0.001215135,
            49.40792,
        ]);
        r_tiff_test_helper(&gt);
    }
}
