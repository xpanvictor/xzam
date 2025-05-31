use crate::audio::decoder::Decoder;

use super::{decoder::DecodedAudio, errors::DecoderError};
use std::path::Path;

pub struct WavDecoder<'a> {
    pub path: &'a Path,
}

impl<'a, P: AsRef<Path>> Decoder<P> for WavDecoder<'a> {
    fn decode(self, path: P) -> super::decoder::TDecodedResult {
        decode_wav(path)
    }
}

pub fn decode_wav<P: AsRef<Path>>(path: P) -> Result<DecodedAudio, DecoderError> {
    let mut reader = hound::WavReader::open(path).unwrap();
    let spec = reader.spec();
    let duration = reader.duration();
    let mut samples = reader.samples::<i32>();

    let mut refined_samples: Vec<f32> = Vec::new();
    let channel_count = spec.channels;
    println!("samplen {}-{channel_count}", samples.len());

    // handle multichannel

    let mut refined_samples = Vec::new();

    loop {
        let sample_chunk = samples
            .by_ref()
            .take(channel_count as usize)
            .collect::<Result<Vec<_>, _>>()?;

        if sample_chunk.is_empty() {
            break; // End of stream
        }

        // Handle partial chunks (end of file) too
        let mono_sum: i32 = sample_chunk.iter().copied().sum();
        let avg = (mono_sum as f32) / sample_chunk.len() as f32;
        refined_samples.push(avg);
    }

    // normalize the sample
    let max: f32 = *refined_samples
        .clone()
        .iter()
        .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
        .unwrap();
    let norm_sample = refined_samples.clone().into_iter().map(|c| c / max);
    println!("Maxxxxxxxxxxxxxxx {max}");

    let decoded = DecodedAudio {
        sample_rate: spec.sample_rate,
        samples: refined_samples,
        channels: channel_count,
        duration_secs: duration as f32,
        normalized_samples: norm_sample.collect(),
    };
    Ok(decoded)
}
