use std::{collections::HashMap, error::Error, path::PathBuf};

use crate::{audio::decoder::decode_audio, db::FingerprintDB, fingerprint::fingerprint_audio};

mod audio;
mod db;
mod fingerprint;

pub type TMatchScore = (String, u32);

// should use config but for now
const DB_PATH: &str = "db/store";

pub fn store_audio(track_id: &str, audio_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut db = FingerprintDB::new(DB_PATH);
    // decode audio
    let decoded_audio = decode_audio(audio_path)?;
    // get audio's fingerprints from
    let f_stream = fingerprint_audio(decoded_audio);
    // for each send to db
    for f_print in f_stream {
        db.save_fingerprint(track_id, &f_print).unwrap();
    }
    Ok(())
}

pub fn score_sample(sample_path: PathBuf) -> Result<TMatchScore, Box<dyn Error>> {
    let db = FingerprintDB::new(DB_PATH);
    // decode audio
    let decoded_audio = decode_audio(sample_path)?;
    // f_prints_stream
    let f_stream = fingerprint_audio(decoded_audio);
    let mut score = HashMap::new();
    for f_print in f_stream {
        let result = db.fetch_matching_fingerprints(&f_print);
        score.extend(result);
    }
    let (track_id, score) = score
        .iter()
        .max_by(|(_, sc_a), (_, sc_b)| sc_a.partial_cmp(sc_b).unwrap())
        .unwrap();
    Ok((track_id.to_owned(), *score))
}
