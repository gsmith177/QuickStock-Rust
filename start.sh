#!/bin/bash

# === BACKEND SETUP ===
echo "🚀 Checking Rust installation..."
if ! command -v rustc &> /dev/null; then
  echo "🛠️ Rust not found. Please install Rust from https://www.rust-lang.org/tools/install"
  exit 1
else
  echo "✅ Rust is installed."
fi

echo "🔧 Building and starting the Rust backend..."
cargo build
cargo run &
BACKEND_PID=$!

# === FRONTEND SETUP ===
FRONTEND_DIR="Frontend"
cd "$FRONTEND_DIR" || { echo "❌ Could not find frontend directory!"; exit 1; }

echo "🚀 Checking Node.js installation..."
if ! command -v node &> /dev/null; then
  echo "🛠️ Node.js not found. Please install Node.js from https://nodejs.org/en"
  exit 1
else
  echo "✅ Node.js is installed."
fi

# Helper function to install a package if not already installed
install_if_missing() {
  PACKAGE="$1"
  if ! npm list "$PACKAGE" >/dev/null 2>&1; then
    echo "📦 Installing $PACKAGE..."
    npm install "$PACKAGE"
  else
    echo "✅ $PACKAGE is already installed."
  fi
}

# Install frontend dependencies
install_if_missing dayjs
install_if_missing react-router-dom
install_if_missing @mui/x-charts@next
install_if_missing @mui/material
install_if_missing @emotion/react
install_if_missing @emotion/styled

# Start frontend
echo "🌐 Starting the React frontend..."
npm start

# Clean up backend process when script is stopped
trap "echo '🛑 Stopping backend...'; kill $BACKEND_PID" EXIT
