//! Low level decoder
//! Converts .wav/.mp3 to pcm samples
//! @author xpanvictor

use crate::audio::wav::WavDecoder;

use super::errors::DecoderError;
use std::path::Path;

#[derive(Debug)]
pub struct DecodedAudio {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    pub duration_secs: f32,
    pub normalized_samples: Vec<f32>,
}

pub type TDecodedResult = Result<DecodedAudio, DecoderError>;

pub trait Decoder<P: AsRef<Path>> {
    fn decode(self, path: P) -> TDecodedResult;
}

pub fn decode_audio<P: AsRef<Path>>(path: P) -> TDecodedResult {
    // Figure type of asset & use appropriate decoder
    let ext: &str = path.as_ref().extension().unwrap().to_str().unwrap();
    let decoder = match ext {
        "wav" => WavDecoder {
            path: path.as_ref(),
        },
        _ => panic!(),
    };
    decoder.decode(&path)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decoder_header() {
        let amapiano_path = Path::new("test_data/amapiano.wav");
        let s = decode_audio(amapiano_path).ok().unwrap();
        println!(
            "Decoded samples {:?} Normalized {:?}",
            s.samples, s.normalized_samples
        )
    }
}
