# DeepSeek Tutor

A Rust-based AI tutoring application that leverages the DeepSeek API to provide intelligent educational assistance. This project demonstrates modern Rust development practices, async programming, and API integration.

## 🎯 Project Overview

### Why This Project Was Built

This project was created to:

1. **Learn Rust in Practice**: Demonstrate real-world Rust development with async programming, error handling, and external API integration
2. **AI Integration**: Showcase how to integrate with AI APIs using Rust's type-safe approach
3. **Educational Tool**: Create a foundation for an AI-powered tutoring system that can be extended for various educational purposes
4. **Modern Development Practices**: Implement proper environment variable management, dependency handling, and project structure

### What It Does

The DeepSeek Tutor is a command-line application that:
- Connects to the DeepSeek AI API
- Sends educational prompts and receives AI-generated responses
- Demonstrates proper error handling and async programming in Rust
- Provides a foundation for building more complex AI tutoring features

## 🏗️ How It Was Built

### Architecture Overview

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Environment   │    │   Rust Client    │    │   DeepSeek API  │
│   Variables     │───▶│   (async-openai) │───▶│   (AI Service)  │
│   (.env file)   │    │                  │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Implementation Details

#### 1. **Async Programming with Tokio**
```rust
#[tokio::main]
async fn main() {
    // Async main function for handling API calls
}
```
- Uses Tokio runtime for async/await support
- Enables non-blocking API calls to DeepSeek

#### 2. **Environment Variable Management**
```rust
use dotenv::dotenv;
use std::env;

dotenv().ok();
let api_key = env::var("OPENAI_API_KEY")
    .expect("OPENAI_API_KEY must be set");
```
- Secure API key management through environment variables
- Uses `dotenv` crate for development convenience
- Proper error handling for missing configuration

#### 3. **API Client Configuration**
```rust
let config = OpenAIConfig::new()
    .with_api_key(api_key);

let client = Client::with_config(config);
```
- Type-safe API client configuration
- Reusable client for multiple requests
- Clean separation of configuration and usage

#### 4. **Error Handling**
```rust
match client.completions().create(request).await {
    Ok(response) => {
        // Handle successful response
    }
    Err(e) => {
        eprintln!("Error calling DeepSeek API: {}", e);
    }
}
```
- Comprehensive error handling for API failures
- User-friendly error messages
- Graceful degradation on errors

## 🚀 How to Run This Project

### Prerequisites

1. **Rust Installation**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **DeepSeek API Key**
   - Sign up at [DeepSeek Platform](https://platform.deepseek.com/)
   - Generate an API key from your dashboard
   - Keep your API key secure and never commit it to version control

### Installation Steps

1. **Clone the Repository**
   ```bash
   git clone <your-repository-url>
   cd deepseek_tutor
   ```

2. **Install Dependencies**
   ```bash
   cargo build
   ```

3. **Configure Environment Variables**
   
   Create a `.env` file in the project root:
   ```bash
   # Create .env file
   touch .env
   ```
   
   Add your configuration:
   ```env
   OPENAI_API_KEY="your_actual_api_key_here"
   BASE_URL="https://api.deepseek.com"
   ```

4. **Run the Application**
   ```bash
   cargo run
   ```

### Development Commands

```bash
# Check for compilation errors
cargo check

# Run with debug output
RUST_LOG=debug cargo run

# Build optimized release
cargo build --release

# Run tests (when implemented)
cargo test
```

## 📦 Dependencies Explained

### Core Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.x | Async runtime for Rust |
| `async-openai` | 0.28 | OpenAI-compatible API client |
| `dotenv` | 0.15 | Environment variable loading |
| `anyhow` | 1.0 | Error handling utilities |

### Why These Dependencies?

- **`tokio`**: Essential for async programming in Rust, provides the runtime for async/await
- **`async-openai`**: Well-maintained crate for OpenAI API compatibility, works with DeepSeek
- **`dotenv`**: Simplifies development by loading environment variables from `.env` files
- **`anyhow`**: Provides convenient error handling patterns for Rust applications

## 🔧 Project Structure

```
deepseek_tutor/
├── src/
│   └── main.rs          # Main application logic
├── .env                 # Environment variables (not tracked)
├── .gitignore          # Git ignore rules
├── Cargo.toml          # Project configuration and dependencies
├── Cargo.lock          # Dependency lock file
└── README.md           # This file
```

## 🛡️ Security Considerations

### Environment Variables
- **Never commit API keys** to version control
- Use `.env` files for local development only
- In production, use proper secret management systems
- The `.env` file is already in `.gitignore`

### API Key Management
```rust
// Secure way to handle API keys
let api_key = env::var("OPENAI_API_KEY")
    .expect("OPENAI_API_KEY must be set");
```

## 🚀 Future Enhancements

This project provides a foundation for building more advanced features:

1. **Interactive Chat Interface**: Add a REPL for ongoing conversations
2. **Multiple AI Models**: Support for different DeepSeek models
3. **Conversation Memory**: Store and recall previous interactions
4. **Educational Modules**: Subject-specific tutoring capabilities
5. **Web Interface**: Add a web frontend using frameworks like Yew or Leptos

## 🐛 Troubleshooting

### Common Issues

1. **"OPENAI_API_KEY must be set"**
   - Ensure your `.env` file exists and contains the correct API key
   - Check that there are no spaces around the `=` in your `.env` file

2. **API Connection Errors**
   - Verify your API key is valid and has sufficient credits
   - Check your internet connection
   - Ensure the DeepSeek API is accessible

3. **Compilation Errors**
   - Run `cargo clean` and `cargo build` to refresh dependencies
   - Ensure you're using a compatible Rust version (1.70+)

### Debug Mode

Run with additional logging:
```bash
RUST_LOG=debug cargo run
```

## 📚 Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Async-OpenAI Documentation](https://docs.rs/async-openai/)
- [DeepSeek API Documentation](https://platform.deepseek.com/api-docs/)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request
