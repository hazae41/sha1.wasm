# sha1.wasm

WebAssembly port of SHA-1

```bash
npm i @hazae41/sha1.wasm
```

[**Node Package 📦**](https://www.npmjs.com/package/@hazae41/sha1.wasm)

## Features
- Reproducible building
- Pre-bundled and streamed
- Zero-copy memory slices

## Modules
- sha1

## Algorithms
- SHA-1 (direct and incremental)

## Usage

### Direct

```typescript
import { Sha1Wasm, Memory, sha1 } from "@hazae41/sha1.wasm";

// Wait for WASM to load
await Sha1Wasm.initBundled();

// Data to be hashed
const hello = new TextEncoder().encode("Hello World")

// Pass to WASM
using memory = new Memory(hello)

// Grab the digest
using digest = sha1(memory)

console.log(digest.bytes) // Uint8Array
```

### Incremental

```typescript
import { Sha1Wasm, Memory, Sha1Hasher } from "@hazae41/sha1.wasm";

// Wait for WASM to load
await Sha1Wasm.initBundled();

// Create a hash
using hasher = new Sha1Hasher()

// Data to be hashed
const hello = new TextEncoder().encode("Hello World")

// Pass to WASM
using memory = new Memory(hello)

// Update the hash with your data
hasher.update(memory)

// Grab the digest
using digest = hasher.finalize()

// Update the hash another time
hasher.update(memory)

// Grab the digest
using digest2 = hasher.finalize()

// digest !== digest2
console.log(digest.bytes)
console.log(digest2.bytes)
```

## Building

### Unreproducible building

You need to install [Rust](https://www.rust-lang.org/tools/install)

Then, install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

```bash
cargo install wasm-pack
```

Finally, do a clean install and build

```bash
npm ci && npm run build
```

### Reproducible building

You can build the exact same bytecode using Docker, just be sure you're on a `linux/amd64` host

```bash
docker compose up --build
```

Then check that all the files are the same using `npm diff`

```bash
npm diff
```

If the output is empty then the bytecode is the same as the one I commited

### Automated checks

Each time I release a new version on GitHub, the GitHub's CI clones the GitHub repository, reproduces the build, and throws an error if the NPM release is different. If a version is present on NPM but not on GitHub, do not use it!
