use std::sync::Arc;

use rustfft::Fft;
use rustfft::num_complex::Complex;

use super::chunker::Frame;
pub type TFftPlanner<T> = Arc<dyn Fft<T>>;

pub trait GenFFT {
    // mapping to magnitude
    fn generate_fft(self: &mut Self, fft: TFftPlanner<f32>) -> Vec<f32>;
}

impl GenFFT for Frame {
    fn generate_fft(self: &mut Frame, fft: TFftPlanner<f32>) -> Vec<f32> {
        let mut cmp_vec = Vec::<Complex<f32>>::from(
            self.chunk
                .iter()
                .map(|v| Complex::new(*v, 0.))
                .collect::<Vec<Complex<f32>>>(),
        );
        // process buffers
        fft.process(&mut cmp_vec);
        // retain only first N/2 bins
        cmp_vec
            .iter()
            .take(cmp_vec.len() / 2)
            .map(|v| v.re.hypot(v.im))
            .collect()
    }
}
