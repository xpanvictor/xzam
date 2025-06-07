use crate::fingerprint::constellation::SpectralPeak;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Fingerprint {
    pub hash: u32,
    pub offset: u32,
}
pub type TFingerprints = Vec<Fingerprint>;

const TIME_INTERVAL_MAX: usize = 5;
const FREQ_THRESHOLD: u32 = 200;

/// Generate hash using bit packing
fn generate_hash(f1: usize, f2: usize, delta_t: usize) -> u32 {
    ((f1 << 22) | (f2 << 12) | delta_t) as u32
}

/// Streaming fingerprint generator: yields fingerprints lazily
pub fn generate_fingerprints<'a>(
    peaks: &'a [SpectralPeak],
) -> impl Iterator<Item = Fingerprint> + 'a {
    peaks.iter().enumerate().flat_map(move |(idx, anchor)| {
        peaks
            .iter()
            .skip(idx + 1)
            .take_while(move |target| {
                target.time_index > anchor.time_index
                    && target.time_index - anchor.time_index < TIME_INTERVAL_MAX
            })
            .map(move |target| Fingerprint {
                hash: generate_hash(
                    anchor.freq_bin,
                    target.freq_bin,
                    target.time_index - anchor.time_index,
                ),
                offset: anchor.time_index as u32,
            })
    })
}
