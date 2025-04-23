#!/bin/bash

# Start backend
echo "🚀 Building and starting the Rust backend..."
cargo build && cargo run &

# Navigate to frontend directory
cd Frontend/frontend/new-frontend

# Install react-router-dom if it's not installed yet
if ! npm list react-router-dom >/dev/null 2>&1; then
  echo "📦 Installing react-router-dom..."
  npm install react-router-dom
fi

# Start frontend
echo "🌐 Starting the React frontend..."
npm start
