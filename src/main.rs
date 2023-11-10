use clap::{Parser, crate_authors, crate_version, crate_description};
use std::path::PathBuf;
use tokio::{self, task};
use tempdir::TempDir;
use futures::future::join_all;
use log::debug;

pub mod gpt;
pub mod whisper;
pub mod util;

#[derive(Parser, Debug)]
#[command(name = "summarize")]
#[command(version = crate_version!())]
#[command(about = crate_description!())]
#[command(author = crate_authors!())]
struct Args {
    /// The path for the file to summarize
    file: PathBuf,
}
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let args = Args::parse();

    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set the OPENAI_API_KEY environment variable");

    let tmp_dir = TempDir::new("audio").expect("Could not create temporary directory");
    let segments = util::split_file(args.file.clone(), &tmp_dir).await.expect("Could not split file");

    
    let mut transcribe_tasks = vec![];

    for segment in segments {
        let client = whisper::WhisperClient::new(api_key.clone());
        let task = task::spawn(async move {
            debug!("Transcribing segment {:?}", segment.index);
            let res = client.transcribe(segment).await;
            
            match res {
                Ok(segment) => {
                    debug!("End segment {:?}", segment.index);
                    return segment
                },
                Err(e) => panic!("Error transcribing segment: {}", e),
            }
        });
        transcribe_tasks.push(task);
    }

    let results: Vec<_> = join_all(transcribe_tasks).await.into_iter().collect();


    tmp_dir.close().expect("Could not delete temporary directory");

    let mut transcribed = Vec::new();

    for result in results {
        match result {
            Ok(segment) => transcribed.push(segment),
            
            Err(e) => eprintln!("Task failed: {}", e),
        }
    }

    let transcript = transcribed.iter().map(|segment| {
        segment.transcript.clone()
    }).collect::<Vec<String>>().join("\n");

    let chunks = util::split_text_to_chunks(transcript);

    let client = gpt::GPTClient::new(api_key);

    let summary = client.summarize(chunks).await.expect("Could not make request to API");

    println!("{}", summary);
}