use thiserror::Error as DeriveError;

#[derive(Debug, DeriveError)]
pub enum Error {
    #[error("Transform is not invertible")]
    TransformNotInvertible,

    #[error("Empty array")]
    EmptyArray,

    #[error(transparent)]
    InvalidLatLng(#[from] h3o::error::InvalidLatLng),

    #[error(transparent)]
    InvalidGeometry(#[from] h3o::error::InvalidGeometry),

    #[error(transparent)]
    InvalidResolution(#[from] h3o::error::InvalidResolution),

    #[error(transparent)]
    CompactionError(#[from] h3o::error::CompactionError),
}
