# rasterh3

[![Latest Version](https://img.shields.io/crates/v/rasterh3.svg)](https://crates.io/crates/rasterh3) [![Documentation](https://docs.rs/rasterh3/badge.svg)](https://docs.rs/rasterh3)

Convert raster data to H3 cells.

Port of [h3ron-ndarray](https://github.com/nmandery/h3ron/tree/main/h3ron-ndarray) from using the h3ron (binding to the
official C implementation) to the rust port [h3o](https://github.com/HydroniumLabs/h3o).
Optional rayon-support using the `rayon` feature.

Also available as a python extension: [h3ronpy](https://github.com/nmandery/h3ronpy)

[Changelog](CHANGES.md)

## Example

See the included `h3ify_r_tiff.rs` for an example how to convert a GeoTIFF read using GDAL.

## License

MIT