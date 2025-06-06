//! The entire spectrogram and fingerprint generation system
//! @author: xpanvictor
//!
//! Algos used:
//!     - windowing
//!     - hanning fn

use std::path::PathBuf;

use crate::{
    audio::decoder::DecodedAudio,
    fingerprint::{constellation::generate_constellation, fingerprint::generate_fingerprints},
};

pub mod chunker;
pub mod constellation;
pub mod fft;
pub mod fingerprint;

const WINDOW_SIZE: u32 = 1024;
const STEP_SIZE: u32 = 512;

pub fn fingerprint_audio(audio: DecodedAudio) -> PathBuf {
    // chunk the audio
    let mut chunks = chunker::chunk_normalized(
        audio.normalized_samples,
        audio.sample_rate,
        WINDOW_SIZE,
        STEP_SIZE,
    );
    // generate constellation
    let constellation = generate_constellation(&mut chunks, WINDOW_SIZE as usize);
    // return fingerprints
    generate_fingerprints(constellation)
}

#[cfg(test)]
mod test {
    use crate::{
        audio::decoder::decode_audio,
        fingerprint::{self, fingerprint_audio},
    };
    use std::path::Path;

    #[test]
    fn generate_fingerprints_from_audio() {
        let amapiano_path = Path::new("test_data/amapiano.wav");
        println!("Generating fingerprints for amapiano song!");
        let s = decode_audio(amapiano_path).ok().unwrap();
        let fingerprints_path = fingerprint_audio(s);
        println!("Fingerprints {:?}", fingerprints_path)
    }
}
