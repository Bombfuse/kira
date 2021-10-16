#[cfg(test)]
mod test;

use crate::Frame;

use super::{PlaybackInfo, Sound, SoundState};

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

	pub fn on_start_processing(&mut self) {
		self.sound.report_playback_info(PlaybackInfo {
			state: self.state,
			position: self.position,
		});
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
