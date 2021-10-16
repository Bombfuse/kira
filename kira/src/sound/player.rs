#[cfg(test)]
mod test;

use crate::Frame;

use super::{Sound, SoundState};

pub(crate) struct SoundPlayer {
	sound: Box<dyn Sound>,
	state: SoundState,
	position: f64,
}

impl SoundPlayer {
	pub fn new(sound: Box<dyn Sound>) -> Self {
		Self {
			sound,
			state: SoundState::Playing,
			position: 0.0,
		}
	}

	pub fn state(&self) -> SoundState {
		self.state
	}

	pub fn process(&mut self, dt: f64) -> Frame {
		let out = self.sound.frame_at_position(self.position);
		self.position += dt;
		if self.position > self.sound.duration().as_secs_f64() {
			self.state = SoundState::Stopped;
		}
		out
	}
}
