pub use rodio::Sink;

use rodio::{Decoder, OutputStream, OutputStreamHandle};

use std::io::BufReader;
use std::fs::File;

pub struct AudioHandler {
	handle: OutputStreamHandle,
	stream: OutputStream,
}

impl AudioHandler {
	pub fn new() -> Self {
		let (stream, handle) = OutputStream::try_default().unwrap();
		Self {
			handle,
			stream,
		}
	}

	pub fn load_wav(path: &str) -> Decoder<BufReader<File>> {
		let file = File::open(path).unwrap();
		Decoder::new_wav(BufReader::new(file)).unwrap()
	}

	pub fn new_sink(&self) -> Sink {
		Sink::try_new(&self.handle).unwrap()
	}
}