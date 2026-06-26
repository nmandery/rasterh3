# Changelog

All notable changes to this project will be documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## v0.12.0 (2026-06-26)

* Fix incorrect value-to-cell mapping for rasters crossing the antimeridian: the longitude shift applied when mapping
  cell centroids back to raster pixels had its sign inverted, and tiles lying fully outside `[-180, 180]` were not
  normalized (caused a debug-only panic and silently wrong cells).
* Fix `ResolutionSearchMode::MinDiff` and `SmallerThanPixel` returning resolution 0 for rasters finer than the smallest
  H3 cell, or when no H3 cell is smaller than a pixel.
* Fix underestimated per-pixel area in `nearest_h3_resolution` for small arrays (the bounding box spanned `shape - 1`
  pixels but was divided by `shape * shape`); 1x1 arrays no longer estimate a zero pixel area.
* Skip cells whose centroid maps to a negative raster coordinate instead of silently clamping them to the first pixel
  via `f64 as usize` saturation.
* Defer the parent-removal pass of `CellCoverage::dedup` to the single final merge step in `to_h3` instead of running it
  once per tile.
* Return `impl Iterator` from the `CellCoverage` iterators instead of `Box<dyn Iterator>`, removing a per-call
  allocation.
* Derive `Default` for `CellCoverage`.
* Benchmark: use `std::hint::black_box` (the `criterion::black_box` alias is deprecated) and convert `r.tiff` with
  `AxisOrder::YX` as GDAL expects.

## v0.11.0 (2026-06-17)

* Upgrade h3o from 0.7 to 0.10
* Bump ndarray to 0.17

## v0.10.0 (2024-11-26)

* Upgrade h3o from 0.6 to 0.7

## v0.9.0 (2024-10-22)

* Bump ndarray to 0.16

## v0.8.0 (2024-06-27)

* Create and expose `AreaOnSphere` trait.
* Replace own affine transformation implementation with `geo::AffineTransform<f64>`.

## v0.7.0 (2024-03-01)

* Fix converting datasets spanning the antimeridian by splitting and normalizing tiles before generating cells from
  them.
* Upgrade h3o from 0.5 to 0.6
* Upgrade geo from 0.26 to 0.27

## v0.6.0 (2024-01-20)

* Upgrade h3o from 0.4 to 0.5
* Feature-gate rayon support. To use multithreading the `rayon` feature now needs to be explicitly enabled.

## v0.5.1 (2023-12-16)

* Upgrade geo from 0.26 to 0.27

## v0.5.0 (2023-08-31)

* Upgrade geo from 0.25 to 0.26
* Upgrade h3o from 0.3 to 0.4

## v0.4.0 (2023-07-04)

* Upgrade geo from 0.23 to 0.24
* Rename `CellSet` struct to `CellCoverage` and expose more methods of its API.

## v0.3.0 (2023-02-12)

* Upgrade h3o to v0.3
* Make `nearest_h3_resolution` function a method of `ResolutionSearchMode`, use a `[usize; 2]` for the shape argument.

## v0.2.0 (2023-01-16)

* Upgrade h3o to v0.2

## v0.1.0 (2023-01-13)

* Initial port of [h3ron-ndarray](https://github.com/nmandery/h3ron/tree/main/h3ron-ndarray) from using the h3ron (
  binding to the official C implementation) to the rust port [h3o](https://github.com/HydroniumLabs/h3o).
