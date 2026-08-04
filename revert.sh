#!/usr/bin/env bash
set -e

# Model tag to use (granite3.1-dense:8b supports tool execution natively in Ollama)
MODEL_TAG="granite3.1-dense:8b"

echo "=== 1. Checking Ollama Daemon ==="
if ! command -v ollama &> /dev/null; then
    echo "Error: Ollama is not installed."
    exit 1
fi

if ! curl -s http://localhost:11434/api/version > /dev/null; then
    echo "Ollama server is not running. Starting Ollama..."
    ollama serve &
    sleep 3
fi

echo "=== 2. Pulling IBM Granite Model: $MODEL_TAG ==="
ollama pull "$MODEL_TAG"

echo "=== 3. Writing Global OpenCode Configuration ==="
CONFIG_DIR="$HOME/.config/opencode"
mkdir -p "$CONFIG_DIR"

cat << EOF > "$CONFIG_DIR/opencode.json"
{
  "\$schema": "https://opencode.ai/config.json",
  "model": "ollama/$MODEL_TAG",
  "provider": {
    "ollama": {
      "npm": "@ai-sdk/openai-compatible",
      "name": "Ollama",
      "options": {
        "baseURL": "http://localhost:11434/v1"
      },
      "models": {
        "$MODEL_TAG": {
          "name": "IBM Granite 3.1 Dense 8B",
          "contextWindow": 131072
        }
      }
    }
  }
}
EOF

echo "=== Setup Complete! ==="
echo "Global configuration written to: $CONFIG_DIR/opencode.json"
echo "You can now launch 'opencode' from any directory."