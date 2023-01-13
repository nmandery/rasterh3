# rasterh3

[![Latest Version](https://img.shields.io/crates/v/h3raster.svg)](https://crates.io/crates/h3raster) [![Documentation](https://docs.rs/h3raster/badge.svg)](https://docs.rs/h3raster)

Convert raster data to H3 cells.

Port of [h3ron-ndarray](https://github.com/nmandery/h3ron/tree/main/h3ron-ndarray) from using the h3ron (binding to the official C implementation) to the rust port [h3o](https://github.com/HydroniumLabs/h3o).

[Changelog](CHANGES.md)

## Example

See the included `h3ify_r_tiff.rs` for an example how to convert a GeoTIFF read using GDAL.

## License

MIT