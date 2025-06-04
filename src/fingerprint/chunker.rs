use std::f32::consts::PI;

struct Frame {
    index: u64,
    chunk: Vec<f32>,
    offset_samples: u64,
    offset_time: u64,
}

pub type TFrames = Vec<Frame>;

pub fn chunk_normalized(
    normalized: Vec<f32>,
    sample_rate: u32,
    window_size: u32,
    step_size: u32,
) -> TFrames {
    let mut frames: TFrames = vec![];
    loop {
        let norm_length = normalized.len();
        let mut index: u64 = 0;
        loop {
            let offset_sample: u64 = index * step_size as u64;

            let curr_buffer: Vec<f32> = normalized.iter().take(window_size as usize).collect();
            let hanned_chunk = hanning(curr_buffer);
            frames.push(Frame {
                index: index,
                chunk: hanned_chunk,
                offset_samples: offset_sample as u64,
                offset_time: offset_sample / sample_rate as u64,
            });
            index += 1;
            if index * window_size as u64 >= norm_length as u64 {
                break;
            }
        }
        break;
    }
    frames
}

pub fn hanning(window: Vec<f32>) -> Vec<f32> {
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
