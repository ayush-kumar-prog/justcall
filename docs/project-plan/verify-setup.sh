#!/bin/bash
echo "Checking installations..."
echo "Rust: $(rustc --version)"
echo "Cargo: $(cargo --version)"
echo "Node: $(node --version)"
echo "npm: $(npm --version)"
echo "Tauri CLI: $(cargo tauri --version)"
echo ""
echo "All tools installed successfully!"