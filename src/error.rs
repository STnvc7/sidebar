use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
	#[error("'{0}' is invalid command")]
	InvalidCommandError(char),
	#[error("Unacceptable key")]
	UnacceptableKeyError,
	#[error("File is already existed")]
	AlreadyExistError,
	#[error("Coudln't run {0} command")]
	SubProcessCommandError(String),
	#[error("Input aborted")]
	InputAbortedError,
}