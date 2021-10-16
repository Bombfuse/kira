pub mod static_sound;

use std::time::Duration;

use crate::{Frame, LoopBehavior};

pub trait Sound {
	fn duration(&mut self) -> Duration;

	fn default_loop_behavior(&mut self) -> Option<LoopBehavior>;

	fn frame_at_position(&mut self, position: f64) -> Frame;
}
