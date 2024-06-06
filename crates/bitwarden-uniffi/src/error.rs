use std::fmt::{Display, Formatter};

// Name is converted from *Error to *Exception, so we can't just name the enum Error because
// Exception already exists
#[derive(uniffi::Error, Debug)]
#[uniffi(flat_error)]
pub enum BitwardenError {
    E(bitwarden::error::Error),
    Ee(bitwarden::exporters::ExportError),
}

impl From<bitwarden::error::Error> for BitwardenError {
    fn from(e: bitwarden::error::Error) -> Self {
        Self::E(e)
    }
}

impl From<bitwarden::exporters::ExportError> for BitwardenError {
    fn from(e: bitwarden::exporters::ExportError) -> Self {
        Self::Ee(e)
    }
}

impl Display for BitwardenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::E(e) => Display::fmt(e, f),
            Self::Ee(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for BitwardenError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            BitwardenError::E(e) => Some(e),
            BitwardenError::Ee(e) => Some(e),
        }
    }
}

pub type Result<T, E = BitwardenError> = std::result::Result<T, E>;
