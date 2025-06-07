//! The entire spectrogram and fingerprint generation system
//! @author: xpanvictor
//!
//! Algos used:
//!     - windowing
//!     - hanning fn

use std::path::PathBuf;

use crate::{
    audio::decoder::DecodedAudio,
    fingerprint::{
        constellation::generate_constellation,
        fingerprint::{Fingerprint, generate_fingerprints},
    },
};

pub mod chunker;
pub mod constellation;
pub mod fft;
pub mod fingerprint;

const WINDOW_SIZE: u32 = 1024;
const STEP_SIZE: u32 = 512;

pub type TFingerprintStream = Box<dyn Iterator<Item = Fingerprint>>;

pub fn fingerprint_audio(audio: DecodedAudio) -> TFingerprintStream {
    // chunk the audio
    let mut chunks = chunker::chunk_normalized(
        audio.normalized_samples,
        audio.sample_rate,
        WINDOW_SIZE,
        STEP_SIZE,
    );
    // generate constellation
    let constellation = generate_constellation(&mut chunks, WINDOW_SIZE as usize);
    // collect fingerprints into a Vec to own the data
    let fingerprints: Vec<Fingerprint> = generate_fingerprints(&constellation).collect();
    Box::new(fingerprints.into_iter())
}

#[cfg(test)]
mod test {
    use crate::{
        audio::decoder::decode_audio,
        fingerprint::{self, fingerprint_audio},
    };
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;

    #[test]
    fn generate_fingerprints_from_audio() {
        let amapiano_path = Path::new("test_data/amapiano.wav");
        let file_id = Uuid::new_v4().to_string();
        let output_path_str = format!("prints/{}.json", file_id);
        let output_path = Path::new(&output_path_str);
        let file = File::create(output_path).expect("Failed to create output file");
        let writer = Arc::new(Mutex::new(file));
        println!("Generating fingerprints for amapiano song!");
        let s = decode_audio(amapiano_path).ok().unwrap();
        let fingerprints = fingerprint_audio(s);

        let mut w = writer.lock().unwrap();
        write!(&mut w, "[").unwrap();
        for f in fingerprints {
            let line = serde_json::to_string(&f).unwrap();
            writeln!(&mut w, "{line},").unwrap();
        }
        writeln!(&mut w, "{{}}]").unwrap();

        println!("Fingerprints stored in {:?}", output_path.to_str())
    }
}
