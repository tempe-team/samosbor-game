<img align="left" height="196" src="screenshot.png?raw=true">

# Samosbor Client

### Requirements:

[`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)<br>
Latest NPM: `npm install npm@latest -g`

---

## Building the WebAssembly part

1. Open `Cargo.toml` and temporary disable the next section at the end:
```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

2. Try to build the WebAssembly code:
```bash
wasm-pack build
```

3. Have no errors? Cool and good. Skip the next step and continue to the next section.

4. Got errors? Look at the command output and figure out your location for the `wasm-opt` executable and comandline it runs with. Copy the commandline somewhere and add the `--enable-mutable-globals` to the end -- you can use this command to run `wasm-opt`. Enable back that section which you disabled at step 1. Now the build process will look like this:
```bash
wasm-pack build # build without optimizations
/home/zezic/.cache/.wasm-pack/wasm-opt-4d7a65327e9363b7/wasm-opt /home/zezic/Documents/Work/Rust/samosbor-client/pkg/samosbor_client_bg.wasm -o /home/zezic/Documents/Work/Rust/samosbor-client/pkg/samosbor_client_bg.wasm-opt.wasm -O --enable-mutable-globals # optimize manually
```

## Building and runnning the web project

```bash
cd www
npm install
npm run start
```

Use the `WS_CONNECT_STRING` environment variable to override default server address which by default is `ws://localhost:8000`:

```bash
WS_CONNECT_STRING="ws://localhost:12345" npm run start
```
