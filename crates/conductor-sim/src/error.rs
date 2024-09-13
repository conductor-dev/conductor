use std::fmt::{Display, Formatter};

pub type ConductorSimResult<T> = Result<T, ConductorSimError>;

#[derive(Debug)]
pub enum ConductorSimError {
    CsvError(csv::Error),
    TcpError(std::io::Error),
    EframeError(eframe::Error),
}

impl From<csv::Error> for ConductorSimError {
    fn from(e: csv::Error) -> Self {
        ConductorSimError::CsvError(e)
    }
}

impl From<std::io::Error> for ConductorSimError {
    fn from(e: std::io::Error) -> Self {
        ConductorSimError::TcpError(e)
    }
}

impl From<eframe::Error> for ConductorSimError {
    fn from(e: eframe::Error) -> Self {
        ConductorSimError::EframeError(e)
    }
}

impl Display for ConductorSimError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ConductorSimError::CsvError(e) => write!(f, "{}", e),
            ConductorSimError::TcpError(e) => write!(f, "{}", e),
            ConductorSimError::EframeError(e) => write!(f, "{}", e),
        }
    }
}
