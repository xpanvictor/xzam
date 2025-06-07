use std::{collections::HashMap, fmt::Error, string};

use bincode::serde;

use crate::fingerprint::{self, fingerprint::Fingerprint};

pub type TTrackElement = (String, u32);
pub type TPrintHashMap = HashMap<(String, i32), usize>;

pub struct FingerprintDB {
    db_path: String,
    db: sled::Db,
}

// HashMap<(track_id, delta), count>

impl FingerprintDB {
    pub fn new(path: &str) -> FingerprintDB {
        FingerprintDB {
            db_path: path.to_owned(),
            db: sled::open(path).unwrap(),
        }
    }

    pub fn save_fingerprint(
        &mut self,
        track_id: &str,
        fingerprint: &Fingerprint,
    ) -> Result<(), Error> {
        let key = fingerprint.hash.to_be_bytes();

        let entry = self.db.get(key).unwrap();
        let mut existing: Vec<(String, u32)> = match entry {
            Some(bytes) => {
                bincode::decode_from_slice(&bytes.as_ref(), bincode::config::standard())
                    .unwrap()
                    .0
            }
            None => Vec::<(String, u32)>::new(),
        };

        existing.push((track_id.to_owned(), fingerprint.offset));

        let encoded = bincode::encode_to_vec(existing, bincode::config::standard()).unwrap();
        self.db.insert(key, encoded).unwrap();
        Ok(())
    }

    pub fn fetch_matching_fingerprints(&self, fingerprint: &Fingerprint) -> Vec<TTrackElement> {
        let output = self.db.get(fingerprint.hash.to_be_bytes()).unwrap();
        if let Some(res) = output {
            // deserialize the vector
            let existing: Vec<(String, u32)> =
                bincode::decode_from_slice(res.as_ref(), bincode::config::standard())
                    .unwrap_or((Vec::new(), 0))
                    .0;
            existing
        } else {
            Vec::new()
        }
    }

    pub fn generate_histogram(&self, fingerprints: &[Fingerprint]) -> TPrintHashMap {
        let mut counts_map: TPrintHashMap = HashMap::new();

        for fingerprint in fingerprints {
            let matches = self.fetch_matching_fingerprints(fingerprint);
            for (track_id, offset) in matches {
                let delta: i32 = fingerprint.offset as i32 - offset as i32;
                *counts_map.entry((track_id, delta)).or_insert(0) += 1;
            }
        }
        counts_map
    }
}

#[cfg(test)]
mod test {
    use std::fs::{self};

    use crate::{db::FingerprintDB, fingerprint::fingerprint::Fingerprint};

    #[test]
    fn check_db_instance() {
        let db = FingerprintDB::new("db/test_db");
        assert!(db.db.is_empty());
        fs::remove_dir_all(db.db_path).unwrap();
    }

    #[test]
    fn check_db_actions() {
        let mut db = FingerprintDB::new("db/test_db");
        let fingerprint = Fingerprint {
            hash: 1234,
            offset: 32,
        };
        let _ = db.save_fingerprint("track", &fingerprint);
        let entries = db.fetch_matching_fingerprints(&fingerprint);
        println!("entries {:?}", entries);
        assert!(
            entries
                .iter()
                .find(|&&(ref id, offset)| id == "track" && offset == fingerprint.offset)
                .is_some()
        );
    }

    #[test]
    fn check_db_histogram() {
        let mut db = FingerprintDB::new("db/test_db");
        let fingerprint = Fingerprint {
            hash: 1234,
            offset: 32,
        };
        let track_id = "track";
        let _ = db.save_fingerprint(track_id, &fingerprint);
        let entries = db.generate_histogram(&[fingerprint]);
        println!("entries {:?}", entries);
        assert!(entries.contains_key(&(track_id.to_owned(), 0)))
    }
}
