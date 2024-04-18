use clap::Parser;
use reqwest::Error;
use serde::Deserialize;
use serde_json::json;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// URL of the server to send the request to
    #[clap(long, default_value = "http://localhost:8081")]
    url: String,

    #[clap(long)]
    input: String,

    #[clap(long, default_value = "true")]
    should_template: bool,
}

#[derive(Deserialize, Debug)]
struct Response {
    generated_text: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    // Create a client
    let client = reqwest::Client::new();

    // JSON data to send with the POST request

    let params = if args.should_template {
        json!({
            "inputs": format!("<s>[INST] {} [/INST]", &args.input),
            "parameters": {
                "max_new_tokens": 100
            }
        })
    } else {
        json!({
            "inputs": &args.input,
            "parameters": {
                "max_new_tokens": 100
            }
        })
    };

    // Send the POST request with JSON body and custom header
    let res = client
        .post(&args.url)
        .json(&params)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if res.status().is_success() {
        let body = res.text().await?;
        // println!("Response text: {}", body);

        // Now try to parse the response text as JSON
        let json_response: Result<Vec<Response>, _> = serde_json::from_str(&body);
        match json_response {
            Ok(parsed) => println!(
                "\n\n========================================\n\nGenerated text: {}",
                parsed[0].generated_text
            ),
            Err(e) => println!("Failed to parse JSON: {:?}", e),
        }
    } else {
        println!("Failed to get an OK response!");
    }

    Ok(())
}
