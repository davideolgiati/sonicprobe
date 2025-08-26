use std::{any::Any, fmt, num::TryFromIntError};

pub struct SonicProbeError {
    pub message: String,
    pub location: String,
}

impl fmt::Display for SonicProbeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.location, self.message)
    }
}

impl fmt::Debug for SonicProbeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SonicProbeError {{ location: {}, message: {} }}",
            self.location, self.message
        )
    }
}

impl From<Box<dyn Any + Send>> for SonicProbeError {
    fn from(panic_payload: Box<dyn Any + Send>) -> Self {
        let message = panic_payload.downcast_ref::<&str>().map_or_else(
            || {
                panic_payload.downcast_ref::<String>().map_or_else(
                    || "Thread panicked with unknown payload".to_owned(),
                    |s| format!("Thread panicked: {s}"),
                )
            },
            |s| format!("Thread panicked: {s}"),
        );

        Self {
            message,
            location: "std::thread::join".to_owned(),
        }
    }
}

impl From<TryFromIntError> for SonicProbeError {
    fn from(error: TryFromIntError) -> Self {
        Self {
            message: format!("Integer conversion failed: {error}"),
            location: "std::num::TryFromIntError".to_owned(),
        }
    }
}

impl From<claxon::Error> for SonicProbeError {
    fn from(error: claxon::Error) -> Self {
        Self {
            message: format!("Claxon error: {error}"),
            location: "claxon::Error".to_owned(),
        }
    }
}