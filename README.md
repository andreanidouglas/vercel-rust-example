---
name: WASM Rust XOR Exclusive Or Example
slug: edge-wasm-rust-xor
description: Perform XOR bitwise operations in your API with Rust and WebAssembly using Edge Functions.
framework: None
useCase: Edge Function
css: None
--

# WASM Rust XOR Exclusive Or Example

Perform XOR bitwise operations in your API with Rust and WebAssembly using Edge Functions.

This examples takes two 32-bit numbers as inputs (the `a` and `b` query parameters) and uses a WASM function written in Rust to exclusive or them.

### Clone and Deploy

Use wasm-pack to bundle the applicatino into a webassembly usable package

```shell
$ cd wasm # rust folder
$ wasm-pack build --target web
```

After creating the `.wasm` and `.js` files, remove any hardcoded references inside
`wasm/wasm.js`. Otherwise vercel will block your package of being published.

Run the app at the root of the repository:

```bash
vercel dev
```

