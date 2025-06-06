use rustfft::num_complex::Complex;

use super::chunker::Frame;

pub trait GenFFT {
    // mapping to magnitude
    fn generate_fft(self: &mut Self) -> Vec<f32>;
}

impl GenFFT for Frame {
    fn generate_fft(self: &mut Frame) -> Vec<f32> {
        let mut cmp_vec = Vec::<Complex<f32>>::from(
            self.chunk
                .iter()
                .map(|v| Complex::new(*v, 0.))
                .collect::<Vec<Complex<f32>>>(),
        );
        let mut fft_planner = rustfft::FftPlanner::new();
        let fft = fft_planner.plan_fft_forward(cmp_vec.len());
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
