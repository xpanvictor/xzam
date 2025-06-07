use clap::{Parser, Subcommand, command};
use xzam::{score_sample, store_audio};

// Clap's configuration
#[derive(Parser)]
#[command(name = "kvs", version, about, author)]
struct Args {
    /// Sub commands for kvs
    #[clap(subcommand)]
    cmd: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    // Add audio to db
    StoreAudio {
        // The name of the audio, use track_id style eg jcole_amara_23
        track_id: String,
        // the path relative to test_data eg amapiano.wav
        track_path: String,
    },
    // Check score of sample
    FindSample {
        // the path relative to test_data eg amapiano_sample.wav
        track_path: String,
    },
}

fn main() {
    let env_vars = Args::parse();

    match env_vars.cmd {
        Subcommands::StoreAudio {
            track_id,
            track_path,
        } => {
            let res = store_audio(&track_id, &track_path);
            match res {
                Ok(()) => {
                    println!("Track - {track_id} saved successfully");
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                }
            }
        }
        Subcommands::FindSample { track_path } => {
            let res = score_sample(&track_path);
            match res {
                Ok((track_id, score)) => {
                    println!(
                        "Winning audio has track_id: {} with score: {}",
                        track_id, score
                    );
                }
                Err(e) => {
                    eprintln!("Unable to score {:?}", e);
                }
            }
        }
    }
}
