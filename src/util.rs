use std::path::PathBuf;
use tempdir::TempDir;
use anyhow::Result;
use std::process::Stdio;

#[derive(Debug)]
pub struct Segment {
    pub index: u32,
    pub path: PathBuf,
    pub transcript: String
}

pub async fn split_file(path: PathBuf, tmp_dir: &TempDir) -> Result<Vec<Segment>, std::io::Error> {

    let mut command = tokio::process::Command::new("ffmpeg")
    .arg("-i")
    .arg(path)
    .arg("-f")
    .arg("segment")
    .arg("-segment_time")
    .arg("180")
    .arg("-c")
    .arg("copy")
    .arg(format!("{}/output%03d.mp3", tmp_dir.path().display()))
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()?;

    let status = command.wait().await?;


    let mut filenames: Vec<PathBuf> = tmp_dir.path().read_dir()?.map(|entry| {
        entry.unwrap().path()
    }).collect();
    filenames.sort();

    let segments: Vec<Segment> = filenames.iter().enumerate().map(|(i, path)| {
        Segment {
            index: i as u32,
            path: path.clone(),
            transcript: String::new()
        }
    }).collect();
    Ok(segments)
}

pub struct Summary {
    pub text: String,
    pub summary: String
}


const WORDS_PER_CHUNK: usize = 2500;

pub fn split_text_to_chunks(text: String) -> Vec<Summary> {
    let mut chunks = Vec::new();
    let mut current_chunk = String::new();
    let mut current_chunk_words = 0;

    for word in text.split_whitespace() {
        current_chunk_words += 1;
        current_chunk.push_str(word);
        current_chunk.push(' ');

        if current_chunk_words >= WORDS_PER_CHUNK {
            chunks.push(current_chunk.clone());
            current_chunk = String::new();
            current_chunk_words = 0;
        }
    }

    if current_chunk_words > 0 {
        chunks.push(current_chunk.clone());
    }

    let mut summaries = Vec::new();
    for chunk in chunks {
        summaries.push(Summary {
            text: chunk.clone(),
            summary: String::new()
        });
    }
    summaries
}