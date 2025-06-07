use std::{collections::HashMap, error::Error, path::Path};

use crate::{audio::decoder::decode_audio, db::FingerprintDB, fingerprint::fingerprint_audio};

mod audio;
mod db;
mod fingerprint;

pub type TMatchScore = (String, u32, i32);

// should use config but for now
const DB_PATH: &str = "db/store";

/// Store audio
/// @param: track_id This takes the id the track should be stored with
/// @param: audio_path The path of the audio, note, place audio in test_data folder
pub fn store_audio(track_id: &str, audio_path_id: &str) -> Result<(), Box<dyn Error>> {
    let mut db = FingerprintDB::new(DB_PATH);
    // decode audio
    let absolute_path = format!("test_data/{}", audio_path_id);
    let audio_path = Path::new(&absolute_path);
    let decoded_audio = decode_audio(audio_path)?;
    // get audio's fingerprints from
    let f_stream = fingerprint_audio(decoded_audio);
    // for each send to db
    for f_print in f_stream {
        db.save_fingerprint(track_id, &f_print).unwrap();
    }
    Ok(())
}

pub fn score_sample(audio_path_id: &str) -> Result<TMatchScore, Box<dyn Error>> {
    let db = FingerprintDB::new(DB_PATH);
    // decode audio
    let absolute_path = format!("test_data/{}", audio_path_id);
    let sample_path = Path::new(&absolute_path);
    let decoded_audio = decode_audio(sample_path)?;
    // f_prints_stream
    let f_stream = fingerprint_audio(decoded_audio);
    let f_vec: Vec<_> = f_stream.collect();
    let score_map = db.generate_histogram(&f_vec);

    let ((track_id, delta), score) = score_map
        .iter()
        .max_by(|(_, sc_a), (_, sc_b)| sc_a.partial_cmp(sc_b).unwrap())
        .unwrap();
    Ok((track_id.to_owned(), *score as u32, *delta))
}
