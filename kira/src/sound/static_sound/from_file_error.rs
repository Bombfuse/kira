use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum FromFileError {
	IoError(std::io::Error),
	SymphoniaError(symphonia::core::errors::Error),
	NoAudio,
	VariableSampleRate,
	UnsupportedChannelConfiguration,
}

impl Display for FromFileError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			FromFileError::IoError(error) => error.fmt(f),
			FromFileError::SymphoniaError(error) => error.fmt(f),
			FromFileError::NoAudio => f.write_str("The file does not contain any audio"),
			FromFileError::VariableSampleRate => f.write_str(
				"The audio has multiple sample rates, which is not supported by StaticSounds",
			),
			FromFileError::UnsupportedChannelConfiguration => {
				f.write_str("Only mono and stereo audio is supported")
			}
		}
	}
}

impl Error for FromFileError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			FromFileError::IoError(error) => Some(error),
			FromFileError::SymphoniaError(error) => Some(error),
			_ => None,
		}
	}
}

impl From<std::io::Error> for FromFileError {
	fn from(v: std::io::Error) -> Self {
		Self::IoError(v)
	}
}

impl From<symphonia::core::errors::Error> for FromFileError {
	fn from(v: symphonia::core::errors::Error) -> Self {
		Self::SymphoniaError(v)
	}
}
