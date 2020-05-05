// error.rs contains the library error type

use thiserror::Error;

// TODO

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid roll format: {0}")]
    RollFormat(String),
}
