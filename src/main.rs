use clap::Parser;
use std::path::PathBuf;

pub mod gpt;
pub mod whisper;

// fn exit(msg: &str) -> ! {
//     let err = clap::Error::print( ErrorKind::InvalidValue);
//     err.exit();
// }

/// CLI app to summarize audio/video files"
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path for the file to summarize
    file: PathBuf,
}

fn main() {
    let args = Args::parse();


    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("Please set the OPENAI_API_KEY environment variable");

    
    let client = whisper::WhisperClient::new(api_key.clone());
    let transcript = client.transcribe(args.file).expect("Could not make request to API");

    let client = gpt::GPTClient::new(api_key);
    let mut response = client.prompt(transcript).expect("Could not make request to API");

    response.push('\n');
    if let Some(r) = response.strip_prefix("\n\n") {
        response = String::from(r);
    }
    println!("{}", response);
}