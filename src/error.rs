use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
	#[error("file is already existed")]
	AlreadyExist,

	#[error("")]
}