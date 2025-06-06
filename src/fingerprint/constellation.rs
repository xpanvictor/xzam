use rustfft::FftPlanner;

use crate::fingerprint::{chunker::TFrames, fft::GenFFT};

pub struct SpectralPeak {
    pub time_index: usize,
    pub freq_bin: usize,
    pub mag: f32,
}

const BAND_SIZE: u32 = 32;

pub fn generate_constellation(frames: &mut TFrames, window_size: usize) -> Vec<SpectralPeak> {
    // fft planner
    let mut fft_planner = FftPlanner::new();
    let fft = fft_planner.plan_fft_forward(window_size);
    frames
        .iter_mut()
        .flat_map(|frame| {
            // generate the fft
            let fft = frame.generate_fft(fft.clone());
            // divide freq spectrum into bands
            // find max
            fft.chunks(BAND_SIZE as usize)
                .enumerate()
                .map(|(i, band)| {
                    let (max_i, max_frq) = band
                        .iter()
                        .enumerate()
                        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                        .unwrap();
                    SpectralPeak {
                        time_index: frame.index as usize,
                        freq_bin: max_i + (i * BAND_SIZE as usize),
                        mag: *max_frq,
                    }
                })
                .collect::<Vec<SpectralPeak>>()
                .into_iter()
        })
        .collect()
}
