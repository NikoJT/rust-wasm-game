# 🦀 Rust WASM Game – Work In Progress

## ✅ Goals
- [x] Create a game loop
- [x] Get a browser-based game working
- [x] Move the character via keys `W`, `A`, `S`, `D`
- [ ] Create jump for the character
- [ ] Create objects with collisions

## 📁 Project Structure:
<pre>
rust-wasm-game/
├── src/              # Rust source files
├── public/
│   ├── index.html    # HTML entry point
│   ├── main.js       # JS glue code
│   ├── assets/       # Sprite images, backgrounds, etc.
│   └── pkg/          # WASM build output (auto-generated)
├── Cargo.toml        # Dependencies
├── README.md         # This readme
└── run.sh            # Build + serve helper script
</pre>

## ⚙️ How to Run:
🛠️ Setup & Run

This project uses Rust and WebAssembly (via wasm-pack) to render and animate a sprite on an HTML canvas.

### 🔧 Prerequisites
Make sure you have the following installed:

	-	[Rust] (https://www.rust-lang.org/tools/install)
	- `wasm-pack` via Cargo
	-	npm (for serving via npx serve)

### 📦 Installing global dependencies

[install-globals]install globals
```bash
   $root cargo install wasm-pack && npm install -g serve
```

#### ▶️ Running the game:
Use run.sh bash script to build and run the wasm. The html is served via serve, which is a requirement to run the script.
```bash
    ./run.sh
```

### Loicense

MIT License

Copyright (c) 2025 Github @NikoJT

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights   
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell      
copies of the Software, and to permit persons to whom the Software is          
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.