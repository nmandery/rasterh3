# Changelog

All notable changes to this project will be documented in this file.

The format is loosely based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).



## Unreleased

## v0.7.0 (2024-03-01)
* Fix converting datasets spanning the antimeridian by splitting and normalizing tiles before generating cells from them.
r Upgrade h3o from 0.5 to 0.6
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
* Initial port of [h3ron-ndarray](https://github.com/nmandery/h3ron/tree/main/h3ron-ndarray) from using the h3ron (binding to the official C implementation) to the rust port [h3o](https://github.com/HydroniumLabs/h3o).
