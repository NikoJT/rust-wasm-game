#!/bin/bash
set -e  # Exit on first error

echo "🧹 Cleaning old pkg..."
rm -rf public/pkg

echo "🔨 Building WASM with wasm-pack..."
wasm-pack build --target web

echo "📦 Moving pkg to public/..."
mv pkg public/

echo "🚀 Serving public/..."
npx serve public/