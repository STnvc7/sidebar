use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
	#[error("Invalid Command")]
	InvalidCommandError,
	#[error("File is already existed")]
	AlreadyExistError,
	#[error("coudln't run shell command")]
	ShellCommandError,
	#[error("Input aborted")]
	InputAbortedError,
}