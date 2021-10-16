use crate::{
	sound::{static_sound::StaticSound, SoundState},
	Frame,
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
