use std::f32::consts::PI;

pub struct Frame {
    pub index: u64,
    pub chunk: Vec<f32>,
    pub offset_samples: u64,
    pub offset_time: u64,
}

pub type TFrames = Vec<Frame>;

fn hanning(window: Vec<f32>) -> Vec<f32> {
    #[allow(non_snake_case)]
    let N = window.len() as f32;
    window
        .iter()
        .map(|val| {
            // apply hanning fn
            0.5 * (1. - ((2. * PI * val) / (N - 1.)).cos())
        })
        .collect()
}

pub fn chunk_normalized(
    normalized: Vec<f32>,
    sample_rate: u32,
    window_size: u32,
    step_size: u32,
) -> TFrames {
    let mut frames: TFrames = vec![];
    let norm_length = normalized.len();
    let mut index: u64 = 0;

    while (index * step_size as u64 + window_size as u64) <= norm_length as u64 {
        let offset_sample = index * step_size as u64;
        let mut curr_buffer: Vec<f32> = normalized
            .iter()
            .skip(offset_sample as usize)
            .take(window_size as usize)
            .cloned()
            .collect();

        // pad if not full
        if curr_buffer.len() < window_size as usize {
            curr_buffer.resize(window_size as usize, 0.);
        }

        let hanned_chunck = self::hanning(curr_buffer);
        frames.push(Frame {
            index,
            chunk: hanned_chunck,
            offset_samples: offset_sample,
            offset_time: offset_sample / sample_rate as u64,
        });
        index += 1;
    }
    frames
}
