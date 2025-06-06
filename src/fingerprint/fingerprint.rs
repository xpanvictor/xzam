use crate::fingerprint::constellation::SpectralPeak;

pub struct Fingerprint {
    hash: u32,
    offset: u32,
}

const TIME_INTERVAL_MAX: usize = 200;
const FREQ_THRESHOLD: u32 = 200;

/// Generate hash using bit packing
fn generate_hash(f1: usize, f2: usize, delta_t: usize) -> u32 {
    ((f1 << 22) | (f2 << 12) | delta_t) as u32
}

pub fn generate_fingerprints(peaks: Vec<SpectralPeak>) -> Vec<Fingerprint> {
    let mut fingerprints = Vec::<Fingerprint>::new();
    for anchor_idx in 0..peaks.len() {
        let anchor = &peaks[anchor_idx];
        let target_zone = peaks
            .iter()
            .skip(anchor_idx)
            .take_while(|p| p.time_index - anchor.time_index < TIME_INTERVAL_MAX);
        let fprints = target_zone.map(|t| Fingerprint {
            hash: generate_hash(
                anchor.freq_bin,
                t.freq_bin,
                t.time_index - anchor.time_index,
            ),
            offset: anchor.time_index as u32,
        });
        fingerprints.extend(fprints);
    }
    fingerprints
}
