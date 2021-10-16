//! Audio data loaded into memory all at once.

mod settings;

pub use settings::*;

#[cfg(test)]
mod test;

use crate::{frame::Frame, loop_behavior::LoopBehavior, util};

use std::{sync::Arc, time::Duration};

use super::Sound;

/// A chunk of audio data loaded into memory all at once.
pub struct StaticSound {
	sample_rate: u32,
	duration: Duration,
	frames: Arc<Vec<Frame>>,
	default_loop_behavior: Option<LoopBehavior>,
}

impl StaticSound {
	/// Creates a new [`StaticSound`] from raw sample data.
	pub fn from_frames(
		sample_rate: u32,
		frames: Vec<Frame>,
		settings: StaticSoundSettings,
	) -> Self {
		let duration = Duration::from_secs_f64(frames.len() as f64 / sample_rate as f64);
		Self {
			sample_rate,
			frames: Arc::new(frames),
			duration,
			default_loop_behavior: settings.default_loop_behavior,
		}
	}
}

impl Sound for StaticSound {
	fn duration(&mut self) -> Duration {
		self.duration
	}

	fn default_loop_behavior(&mut self) -> Option<LoopBehavior> {
		self.default_loop_behavior
	}

	fn frame_at_position(&mut self, position: f64) -> Frame {
		let sample_position = self.sample_rate as f64 * position;
		let fraction = (sample_position % 1.0) as f32;
		let current_sample_index = sample_position as usize;
		let previous = if current_sample_index == 0 {
			Frame::ZERO
		} else {
			*self
				.frames
				.get(current_sample_index - 1)
				.unwrap_or(&Frame::ZERO)
		};
		let current = *self
			.frames
			.get(current_sample_index)
			.unwrap_or(&Frame::ZERO);
		let next_1 = *self
			.frames
			.get(current_sample_index + 1)
			.unwrap_or(&Frame::ZERO);
		let next_2 = *self
			.frames
			.get(current_sample_index + 2)
			.unwrap_or(&Frame::ZERO);
		util::interpolate_frame(previous, current, next_1, next_2, fraction)
	}
}
