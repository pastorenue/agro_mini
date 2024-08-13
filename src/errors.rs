use std::fmt;
use collections::HashMap;


enum AgroException {
    InvalidInput,
    InvalidFarmError(String),
    InvalidCropError(String),
    FarmNotFound,
    Unauthorized(String),
    InvalidSeasonCropError(String, Option<HashMap<String, String>>),
    PermissionDenied,
}

impl fmt::Display for AgroException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgroException::InvalidInput => write!(f, "Invalid input"),
            AgroException::InvalidFarmError(msg) => write!(f, "Invalid farm: {}", msg),
            AgroException::InvalidCropError(msg) => write!(f, "Invalid crop: {}", msg),
            AgroException::FarmNotFound => write!(f, "Farm not found"),
            AgroException::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AgroException::InvalidSeasonCropError(msg, params) => {
                write!(f, "Invalid season crop: {} with params: {}", msg, params)
            },
            AgroException::PermissionDenied => write!(f, "Permission denied"),
        }
    }
}

impl std::error::Error for AgroException {}
