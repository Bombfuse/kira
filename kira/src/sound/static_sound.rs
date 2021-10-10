//! Audio data loaded into memory all at once.

mod settings;
pub use settings::*;

#[cfg(feature = "symphonia")]
mod from_file_error;
#[cfg(feature = "symphonia")]
pub use from_file_error::*;

use crate::{frame::Frame, loop_behavior::LoopBehavior, util};

use std::{path::Path, time::Duration};

use super::Sound;

/// A chunk of audio data loaded into memory all at once.
pub struct StaticSound {
	sample_rate: u32,
	duration: Duration,
	frames: Vec<Frame>,
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
			frames,
			duration,
			default_loop_behavior: settings.default_loop_behavior,
		}
	}

	#[cfg(feature = "symphonia")]
	/// Creates a new [`StaticSound`] from an audio file.
	pub fn from_file(
		path: impl AsRef<Path>,
		settings: StaticSoundSettings,
	) -> Result<Self, FromFileError> {
		fn inner(path: &Path, settings: StaticSoundSettings) -> Result<StaticSound, FromFileError> {
			use std::{fs::File, io::ErrorKind};

			use symphonia::core::{
				audio::{AudioBuffer, AudioBufferRef, Signal},
				conv::IntoSample,
				errors::Error,
				io::MediaSourceStream,
				probe::Hint,
				sample::Sample,
			};

			fn read_frames_from_buffer<T: Sample + IntoSample<f32>>(
				frames: &mut Vec<Frame>,
				buffer: &AudioBuffer<T>,
			) -> Result<(), FromFileError> {
				let num_channels = buffer.spec().channels.count();
				if !(num_channels == 1 || num_channels == 2) {
					return Err(FromFileError::UnsupportedChannelConfiguration);
				}
				for i in 0..buffer.frames() {
					match num_channels {
						1 => frames.push(Frame::from_mono(buffer.chan(0)[i].into_sample())),
						2 => frames.push(Frame::new(
							buffer.chan(0)[i].into_sample(),
							buffer.chan(1)[i].into_sample(),
						)),
						_ => unreachable!(),
					}
				}
				Ok(())
			}

			fn read_frames_from_buffer_ref(
				frames: &mut Vec<Frame>,
				buffer_ref: &AudioBufferRef,
			) -> Result<(), FromFileError> {
				match buffer_ref {
					AudioBufferRef::U8(buffer) => read_frames_from_buffer(frames, buffer),
					AudioBufferRef::U16(buffer) => read_frames_from_buffer(frames, buffer),
					AudioBufferRef::U24(buffer) => read_frames_from_buffer(frames, buffer),
					AudioBufferRef::U32(buffer) => read_frames_from_buffer(frames, buffer),
					AudioBufferRef::S8(buffer) => read_frames_from_buffer(frames, buffer),
					AudioBufferRef::S16(buffer) => read_frames_from_buffer(frames, buffer),
					AudioBufferRef::S24(buffer) => read_frames_from_buffer(frames, buffer),
					AudioBufferRef::S32(buffer) => read_frames_from_buffer(frames, buffer),
					AudioBufferRef::F32(buffer) => read_frames_from_buffer(frames, buffer),
					AudioBufferRef::F64(buffer) => read_frames_from_buffer(frames, buffer),
				}
			}

			let file = File::open(path)?;
			let codec_registry = symphonia::default::get_codecs();
			let probe = symphonia::default::get_probe();
			let media_source_stream = MediaSourceStream::new(Box::new(file), Default::default());
			let probe_result = probe.format(
				&Hint::new(),
				media_source_stream,
				&Default::default(),
				&Default::default(),
			)?;
			let mut format_reader = probe_result.format;
			let track = format_reader
				.default_track()
				.ok_or(FromFileError::NoAudio)?;
			let mut decoder = codec_registry.make(&track.codec_params, &Default::default())?;
			let mut frames = vec![];
			let mut sample_rate = None;
			loop {
				match format_reader.next_packet() {
					Ok(packet) => {
						let buffer_ref = decoder.decode(&packet)?;
						// if we've decoded a previous packet, make sure the sample rate
						// of this packet matches (StaticSounds don't currently support
						// variable sample rates). otherwise, set the sample rate now
						if let Some(sample_rate) = sample_rate {
							if sample_rate != buffer_ref.spec().rate {
								return Err(FromFileError::VariableSampleRate);
							}
						} else {
							sample_rate = Some(buffer_ref.spec().rate);
						}
						read_frames_from_buffer_ref(&mut frames, &buffer_ref)?;
					}
					Err(Error::IoError(err)) => {
						// UnexpectedEof is how we know we've reached the end of the
						// audio, so break out of the loop. any other error is a
						// legitimate error
						if err.kind() == ErrorKind::UnexpectedEof {
							break;
						} else {
							return Err(Error::IoError(err).into());
						}
					}
					Err(err) => return Err(err.into()),
				}
			}
			// if we haven't determined a sample rate yet, there must have been no
			// packets, so return the NoAudio error
			let sample_rate = sample_rate.ok_or(FromFileError::NoAudio)?;
			let duration = Duration::from_secs_f64(frames.len() as f64 / sample_rate as f64);
			Ok(StaticSound {
				sample_rate,
				duration,
				frames,
				default_loop_behavior: settings.default_loop_behavior,
			})
		}

		inner(path.as_ref(), settings)
	}
}

impl Sound for StaticSound {
	fn duration(&self) -> Duration {
		self.duration
	}

	fn frame_at_position(&self, position: f64) -> Frame {
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

	fn default_loop_behavior(&self) -> Option<LoopBehavior> {
		self.default_loop_behavior
	}
}
