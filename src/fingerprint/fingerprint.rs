use crate::fingerprint::constellation::SpectralPeak;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Fingerprint {
    hash: u32,
    offset: u32,
}
pub type TFingerprints = Vec<Fingerprint>;

const TIME_INTERVAL_MAX: usize = 5;
const FREQ_THRESHOLD: u32 = 200;

/// Generate hash using bit packing
fn generate_hash(f1: usize, f2: usize, delta_t: usize) -> u32 {
    ((f1 << 22) | (f2 << 12) | delta_t) as u32
}

pub fn generate_fingerprints(peaks: Vec<SpectralPeak>) -> PathBuf {
    let file_id = Uuid::new_v4().to_string();
    let output_path_str = format!("prints/{}.json", file_id);
    let output_path = Path::new(&output_path_str);
    let file = File::create(output_path).expect("Failed to create output file");
    let writer = Arc::new(Mutex::new(file));

    peaks.par_iter().enumerate().for_each(|(idx, anchor)| {
        let mut buffer = Vec::new();
        let target_zone = peaks
            .iter()
            .skip(idx)
            .take_while(|p| p.time_index - anchor.time_index < TIME_INTERVAL_MAX);
        for target in target_zone {
            let p = Fingerprint {
                hash: generate_hash(
                    anchor.freq_bin,
                    target.freq_bin,
                    target.time_index - anchor.time_index,
                ),
                offset: anchor.time_index as u32,
            };
            buffer.push(p);
        }

        // write to file
        let mut w = writer.lock().unwrap();
        for f in buffer {
            let line = serde_json::to_string(&f).unwrap();
            writeln!(&mut w, "{line}").unwrap();
        }
    });
    output_path.to_path_buf()
}
