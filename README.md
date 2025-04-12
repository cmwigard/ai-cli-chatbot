# Rust CLI Chatbot

A simple CLI-based AI chatbot built in Rust that interacts with a local API server, streaming responses token-by-token for a smooth, real-time experience.

---

## Configuration

The chatbot expects a `config.toml` file at the root of the project. Example:

```toml
[settings]
api = "http://localhost:11434/api/chat"
model = "mistral"
```

- `api`: The API endpoint where your chatbot model is running.
- `model`: The model name you want to use.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- A locally running or accessible chatbot API

  If you're using [Ollama](https://ollama.com), make sure it is installed and running. You can start a model by running:

  ```bash
  ollama run mistral
  ```

  This command will start a server at `http://localhost:11434` and load the `mistral` model. You can change the model name based on your configuration.

### Running the Project

```bash
# Clone the repo
git clone https://github.com/cmwigard/ai-cli-chatbot.git
cd ai-cli-chatbot

# Build the project
cargo build

# Run the chatbot
cargo run
```

---

## Usage

Once the bot is running:

```bash
> Hello there!
Hello! How can I help you today?

> exit
Goodbye!
```

The assistant streams responses word-by-word or token-by-token for a fluid interaction.

---

## Features

- Streamed token-by-token output
- Configurable model and API endpoint via TOML
- Modular code structure (API, config, error handling)
- Custom error types with `thiserror`
- Async I/O with `tokio`

---

## License

© Copyright Chris Martin Wigard 2025

This software is provided “as is”, without warranty of any kind.
You may use, copy, and modify it for personal or educational purposes only.
Commercial use, redistribution, or sublicensing is prohibited without prior written permission.

---

