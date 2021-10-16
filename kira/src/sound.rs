pub(crate) mod player;
pub mod static_sound;

use std::time::Duration;

use atomic_arena::Index;

use crate::{Frame, LoopBehavior};

/// A unique identifier for a sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundId(pub(crate) Index);

/// The playback state of a sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundState {
	/// The sound is playing.
	Playing,
	/// The sound is fading out, and when the fade-out
	/// is finished, the sound will pause playback.
	Pausing,
	/// The sound is paused.
	Paused,
	/// The sound is fading out, and when the fade-out
	/// is finished, the sound will stop.
	Stopping,
	/// The sound is stopped and cannot be interacted with
	/// further.
	Stopped,
}

impl SoundState {
	fn from_u8(value: u8) -> Self {
		match value {
			0 => Self::Playing,
			1 => Self::Pausing,
			2 => Self::Paused,
			3 => Self::Stopping,
			4 => Self::Stopped,
			_ => panic!("{} is not a valid SoundState", value),
		}
	}

	fn is_playing(&self) -> bool {
		matches!(
			self,
			SoundState::Playing | SoundState::Pausing | SoundState::Stopping
		)
	}
}

pub trait Sound: Send + Sync {
	fn duration(&mut self) -> Duration;

	fn default_loop_behavior(&mut self) -> Option<LoopBehavior>;

	fn frame_at_position(&mut self, position: f64) -> Frame;
}
