1. Build a0/cspr

```bash
wasm-pack build
```

2. Make sure that all provided `package.json` has `"type": "module"`, especially in the pkg directory.

3. Check wasm

```bash
 node --experimental-wasm-modules index.js
```
