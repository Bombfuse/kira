use crate::{manager::resources::mixer::Mixer, track::TrackId};

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

	pub fn process(&mut self, dt: f64, mixer: &mut Mixer) {
		if let Some(track) = mixer.track_mut(TrackId::Main) {
			track.add_input(self.sound.frame_at_position(self.position));
			self.position += dt;
			if self.position > self.sound.duration().as_secs_f64() {
				self.state = SoundState::Stopped;
			}
		}
	}
}
