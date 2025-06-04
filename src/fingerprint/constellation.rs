use crate::fingerprint::{chunker::TFrames, fft::GenFFT};

pub type TSpectralPeak = Vec<f32>;

pub fn generate_constellation(frames: &mut TFrames) -> TSpectralPeak {
    frames
        .iter_mut()
        .map(|frame| {
            // generate the fft
            let fft = frame.generate_fft();
            // find max
            return fft[0];
        })
        .collect()
}
