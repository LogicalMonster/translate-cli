use anyhow::Result;
use std::io::{self, BufRead};
use reqwest::Client;
use serde::Deserialize;
use config::Config;
use clap::Parser;

const TRANSLATE_URL: &str = "https://translation.googleapis.com/language/translate/v2";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 目标语言代码 (例如: zh-CN, en, ja)
    #[arg(short, long, default_value = "zh-CN")]
    target: String,
}

#[derive(Debug, Deserialize)]
struct Settings {
    api_key: String,
}

#[derive(Debug, Deserialize)]
struct TranslateResponse {
    data: TranslateData,
}

#[derive(Debug, Deserialize)]
struct TranslateData {
    translations: Vec<Translation>,
}

#[derive(Debug, Deserialize)]
struct Translation {
    #[serde(rename = "translatedText")]
    translated_text: String,
    #[serde(rename = "detectedSourceLanguage")]
    detected_source_language: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?
        .try_deserialize::<Settings>()?;

    let client = Client::new();
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let text = line?;
        if !text.trim().is_empty() {
            match translate_text(&client, &text, &settings.api_key, &args.target).await {
                Ok(translated) => println!("{}", translated),
                Err(e) => eprintln!("Translation Errors: {}", e),
            }
        }
    }
    Ok(())
}

async fn translate_text(client: &Client, text: &str, api_key: &str, target: &str) -> Result<String> {
    let response = client
        .post(TRANSLATE_URL)
        .query(&[("key", api_key)])
        .json(&serde_json::json!({
            "q": text,
            "target": target,
            "format": "text"
        }))
        .send()
        .await?
        .error_for_status()?
        .json::<TranslateResponse>()
        .await?;

    let translation = &response.data.translations[0];
    if let Some(lang) = &translation.detected_source_language {
        eprintln!("Source language: {}", lang);
    }

    Ok(translation.translated_text.clone())
}
