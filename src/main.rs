use dotenv::dotenv;
use std::env;
use async_openai::{Client, config::OpenAIConfig};
#[tokio::main]
async fn main() {
dotenv().ok();

//Get API key from enviroment variables
let api_key = env::var("OPENAI_API_KEY")
    .expect("OPENAI_API_KEY must be set");
let base_url=env::var("BASE_URL")
    .expect("BASE_URL must be set");

println!("API Key: {}", api_key);
println!("Base URL: {}", base_url);

//create config with explicit values
let config = OpenAIConfig::new()
    .with_api_key(api_key);

//Initialize the OpenAi client with config
let client = Client::with_config(config);

println!("Client initialized successfully!");

// Make a simple completion request (not chat completion)
let request = async_openai::types::CreateCompletionRequestArgs::default()
    .model("gpt-4o-mini")
    .prompt("Hello! Can you tell me a short joke?")
    .max_tokens(100u32)
    .build()
    .unwrap();

match client.completions().create(request).await {
    Ok(response) => {
        if let Some(choice) = response.choices.first() {
            println!("DeepSeek Response: {}", choice.text);
        }
    }
    Err(e) => {
        eprintln!("Error calling DeepSeek API: {}", e);
    }
}
}
