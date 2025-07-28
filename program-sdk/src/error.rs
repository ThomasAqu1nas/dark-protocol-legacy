// use std::{
//     backtrace::Backtrace,
//     fmt::{self, Display, Formatter},
//     io,
// };

// pub type ProgramSdkResult<T> = Result<T, ProgramSdkError>;

// /// Внутренняя ошибка сервиса пруфера
// #[derive(Debug)]
// pub enum ProverServiceError {
//     OpenSetupFileError {
//         source: io::Error,
//         backtrace: Backtrace,
//     },
// }

// impl ProverServiceError {
//     #[track_caller]
//     pub fn open_setup_file(err: io::Error) -> Self {
//         ProverServiceError::OpenSetupFileError {
//             source: err,
//             backtrace: Backtrace::capture(),
//         }
//     }
// }

// impl Display for ProverServiceError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             ProverServiceError::OpenSetupFileError { source, backtrace } => {
//                 writeln!(f, "OpenSetupFileError: {}", source)?;
//                 write!(f, "Backtrace:\n{}", backtrace)
//             }
//         }
//     }
// }

// impl std::error::Error for ProverServiceError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             ProverServiceError::OpenSetupFileError { source, .. } => Some(source),
//         }
//     }
// }

// /// Внешняя ошибка SDK
// #[derive(Debug)]
// pub enum ProgramSdkError {
//     ProverServiceError(ProverServiceError),
//     // … другие категории ошибок
// }

// impl Display for ProgramSdkError {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         match self {
//             ProgramSdkError::ProverServiceError(e) => write!(f, "ProverServiceError({})", e),
//         }
//     }
// }

// impl std::error::Error for ProgramSdkError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             ProgramSdkError::ProverServiceError(e) => Some(e),
//         }
//     }
// }

// impl From<ProverServiceError> for ProgramSdkError {
//     fn from(e: ProverServiceError) -> Self {
//         ProgramSdkError::ProverServiceError(e)
//     }
// }

// impl From<io::Error> for ProverServiceError {
//     fn from(err: io::Error) -> Self {
//         ProverServiceError::open_setup_file(err)
//     }
// }

use thiserror::Error;

pub type ProgramSdkResult<T> = std::result::Result<T, ProgramSdkError>;

/// Все ошибки вашего SDK
#[derive(Debug, Error)]
pub enum ProgramSdkError {
    #[error("solana RPC error: {0}")]
    RpcError(#[from] solana_client::client_error::ClientError),

    #[error("transaction build error: {0}")]
    TransactionError(#[from] solana_sdk::transaction::TransactionError),

    #[error("instruction error: {0}")]
    InstructionError(#[from] solana_sdk::program_error::ProgramError),

    #[error("signer error: {0}")]
    SignerError(#[from] solana_sdk::signer::SignerError),

    #[error("compile error: {0}")]
    CompileError(#[from] solana_sdk::message::CompileError),

    #[error("serialization error: {0}")]
    SerializationError(#[from] ark_serialize::SerializationError),

    #[error("synthesis error")]
    SynthesisError(#[from] ark_relations::r1cs::SynthesisError),

    #[error("circom config creation error")]
    CircomConfigError(#[from] color_eyre::Report),

    #[error("transparent")]
    Other(#[from] anyhow::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    // … сюда можно добавить ещё, если понадобятся другие источники ошибок
}
