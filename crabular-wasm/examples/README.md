# Examples

## Browser

```bash
cd ..
wasm-pack build --target web
npx serve .
# Open http://localhost:3000/examples/
```

**Note:** Serve from `crabular-wasm` directory, not `examples/`. Web servers cannot access parent directories.

## Node.js

```bash
cd ..
wasm-pack build --target nodejs
cd examples
node node-demo.js
```

## Build Targets

```bash
# Web (ES modules)
wasm-pack build --target web

# Node.js
wasm-pack build --target nodejs

# Bundlers (webpack, rollup)
wasm-pack build --target bundler
```
