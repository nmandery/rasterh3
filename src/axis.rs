/// The order of the axis in the two-dimensional array
#[derive(Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum AxisOrder {
    /// `X,Y` ordering
    XY,

    /// `Y,X` ordering
    ///
    /// This is the order used by [github.com/georust/gdal](https://github.com/georust/gdal) (behind the `ndarray` feature gate)
    YX,
}

impl AxisOrder {
    pub const fn x_axis(&self) -> usize {
        match self {
            Self::XY => 0,
            Self::YX => 1,
        }
    }

    pub const fn y_axis(&self) -> usize {
        match self {
            Self::XY => 1,
            Self::YX => 0,
        }
    }
}
