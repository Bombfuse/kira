use atomic_arena::{Arena, Controller};
use ringbuf::Producer;

use crate::{
	manager::command::SoundCommand,
	sound::{player::SoundPlayer, SoundState},
};

use super::mixer::Mixer;

pub(crate) struct SoundPlayers {
	sound_players: Arena<SoundPlayer>,
	unused_sound_player_producer: Producer<SoundPlayer>,
}

impl SoundPlayers {
	pub(crate) fn new(
		capacity: usize,
		unused_sound_player_producer: Producer<SoundPlayer>,
	) -> Self {
		Self {
			sound_players: Arena::new(capacity),
			unused_sound_player_producer,
		}
	}

	pub fn controller(&self) -> Controller {
		self.sound_players.controller()
	}

	pub fn on_start_processing(&mut self) {
		self.remove_unused_sound_players();
	}

	fn remove_unused_sound_players(&mut self) {
		if self.unused_sound_player_producer.is_full() {
			return;
		}
		for (_, sound_player) in self
			.sound_players
			.drain_filter(|sound_player| sound_player.state() == SoundState::Stopped)
		{
			if self
				.unused_sound_player_producer
				.push(sound_player)
				.is_err()
			{
				panic!("Unused sound player producer is full")
			}
			if self.unused_sound_player_producer.is_full() {
				return;
			}
		}
	}

	pub fn run_command(&mut self, command: SoundCommand) {
		match command {
			SoundCommand::Add(id, sound_player) => {
				self.sound_players
					.insert_with_index(id.0, sound_player)
					.expect("Sound player arena is full");
			}
		}
	}

	pub fn process(&mut self, dt: f64, mixer: &mut Mixer) {
		for (_, sound_player) in &mut self.sound_players {
			sound_player.process(dt, mixer);
		}
	}
}
