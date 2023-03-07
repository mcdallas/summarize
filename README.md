
## About

A simple CLI tool to summarize audio files. Uses the OpenAI whisper API to transcribe the audio to text and then the OpenAI ChatGPT API to summarize the text.

## Installation

#### Using cargo

If you already have Rust installed, you can install using `cargo` 
```shell script
cargo install summarize
```

#### Using homebrew

```shell script
brew tap mcdallas/summarize
brew install summarize
```

#### Github Release

just grab the [latest release](https://github.com/mcdallas/summarize/releases/latest) for your OS 



## Usage

The `$OPENAI_API_KEY` environmental variable is required. You can create an API key at https://platform.openai.com/account/api-keys

``` shell
# download an audio file
> yt-dlp --extract-audio https://www.youtube.com/watch\?v\=hzSbR4BeOOM --audio-format mp3 -o buffet.mp3
[youtube] Extracting URL: https://www.youtube.com/watch?v=hzSbR4BeOOM
[youtube] hzSbR4BeOOM: Downloading webpage
[youtube] hzSbR4BeOOM: Downloading android player API JSON
[info] hzSbR4BeOOM: Downloading 1 format(s): 251
[download] Destination: buffet.webm
[download] 100% of   10.94MiB in 00:00:01 at 10.08MiB/s
[ExtractAudio] Destination: buffet.mp3
Deleting original file buffet.webm (pass -k to keep)

# summarize the audio file
> summarize buffet.mp3
Volatility does not measure risk, according to Warren Buffett and Charlie Munger.
They argue that volatility, as measured by data, is mathematically useful but wrong when it comes to risk.
Instead, risk should be assessed based on the nature of the business, capital structure, and the price paid.
They criticize the notion of risk-adjusted returns and say that finance professors have mixed up 
volatility with risk, leading to a lot of "foolish mathematics." The key to successful investing is understanding the 
economics of the business and not paying too much to buy into it. 
The stock market's volatility is an advantage for investors as it creates more opportunities for mispricing

```