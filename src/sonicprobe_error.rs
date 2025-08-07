use std::fmt;

pub struct SonicProbeError {
    pub message: String,
    pub location: String,
}

impl fmt::Display for SonicProbeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.location, self.message)
    }
}

// A unique format for dubugging output
impl fmt::Debug for SonicProbeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SonicProbeError {{ location: {}, message: {} }}",
            self.location, self.message
        )
    }
}
