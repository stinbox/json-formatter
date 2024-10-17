# sample app for wasm build of `json_formatter`

## build

On the top of `json_formatter` ( e.g. `..` ),

```sh
rm -rf ./app/pkg; wasm-pack build --target web --out-dir ./app/pkg
```

## run

This sample uses `<script type="module">` and fails to load by `file://` for CORS policy of browsers.

For example, use VSCode `Live Server` or `python3 -m http.server`.
