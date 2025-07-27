use std::fmt::Display;

pub enum ProgramSdkError {
    ProverServiceError(ProverServiceError),
}

pub enum ProverServiceError {
    OpenSetupFileError,
}

impl Display for ProverServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProverServiceError::OpenSetupFileError =>
                write!(f, "ProverServiceError::OpenSetupFileError"),
        }
    }
}

impl Display for ProgramSdkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramSdkError::ProverServiceError(prover_service_error) =>
                write!(f, "ProverServiceError: {}", prover_service_error),
        }
    }
}
