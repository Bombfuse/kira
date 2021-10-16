use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

use crate::{
	sound::{static_sound::StaticSound, PlaybackInfo, Sound, SoundState},
	Frame, LoopBehavior,
};

use super::SoundPlayer;

#[test]
fn playback() {
	let mut player = SoundPlayer::new(Box::new(StaticSound::from_frames(
		1,
		vec![
			Frame::from_mono(0.0),
			Frame::from_mono(1.0),
			Frame::from_mono(2.0),
			Frame::from_mono(3.0),
		],
		Default::default(),
	)));
	assert_eq!(player.state(), SoundState::Playing);
	assert_eq!(player.process(1.0), Frame::from_mono(0.0));
	assert_eq!(player.state(), SoundState::Playing);
	assert_eq!(player.process(1.0), Frame::from_mono(1.0));
	assert_eq!(player.state(), SoundState::Playing);
	assert_eq!(player.process(1.0), Frame::from_mono(2.0));
	assert_eq!(player.state(), SoundState::Playing);
	assert_eq!(player.process(1.0), Frame::from_mono(3.0));
	assert_eq!(player.state(), SoundState::Playing);
	assert_eq!(player.process(1.0), Frame::from_mono(0.0));
	assert_eq!(player.state(), SoundState::Stopped);
}

struct DummySound {
	last_reported_playback_info: Arc<Mutex<Option<PlaybackInfo>>>,
}

impl DummySound {
	fn new() -> Self {
		Self {
			last_reported_playback_info: Arc::new(Mutex::new(None)),
		}
	}
}

impl Sound for DummySound {
	fn duration(&mut self) -> Duration {
		Duration::from_secs(10)
	}

	fn default_loop_behavior(&mut self) -> Option<LoopBehavior> {
		None
	}

	fn report_playback_info(&mut self, playback_info: PlaybackInfo) {
		*self.last_reported_playback_info.lock().unwrap() = Some(playback_info);
	}

	fn frame_at_position(&mut self, _position: f64) -> Frame {
		Frame::ZERO
	}
}

#[test]
fn playback_info_reporting() {
	let sound = DummySound::new();
	let last_reported_playback_info = sound.last_reported_playback_info.clone();
	let mut player = SoundPlayer::new(Box::new(sound));
	player.on_start_processing();
	{
		let playback_info = last_reported_playback_info.lock().unwrap();
		assert_eq!(
			*playback_info,
			Some(PlaybackInfo {
				state: SoundState::Playing,
				position: 0.0,
			})
		)
	}
	player.process(2.0);
	player.on_start_processing();
	{
		let playback_info = last_reported_playback_info.lock().unwrap();
		assert_eq!(
			*playback_info,
			Some(PlaybackInfo {
				state: SoundState::Playing,
				position: 2.0,
			})
		)
	}
	player.process(10.0);
	player.on_start_processing();
	{
		let playback_info = last_reported_playback_info.lock().unwrap();
		assert_eq!(
			*playback_info,
			Some(PlaybackInfo {
				state: SoundState::Stopped,
				position: 12.0,
			})
		)
	}
}
