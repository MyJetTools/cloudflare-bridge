use super::contracts::ErrorContract;

#[derive(Debug)]
pub enum CloudFlareError {
    FlUrlError(flurl::FlUrlError),
    RecordAlreadyExists(String),
    UnknownError(String),
}

impl From<flurl::FlUrlError> for CloudFlareError {
    fn from(error: flurl::FlUrlError) -> Self {
        CloudFlareError::FlUrlError(error)
    }
}

impl Into<CloudFlareError> for ErrorContract {
    fn into(self) -> CloudFlareError {
        match self.code {
            81057 => CloudFlareError::RecordAlreadyExists(self.message),
            _ => CloudFlareError::UnknownError(self.message),
        }
    }
}
