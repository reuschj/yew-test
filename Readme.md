# Yew WASM Test
This just a simple trial at building a WASM web app in Rust/Yew without JavaScript. The the app is an experiment and is not intended to serve any real useful purpose.

## The App
The app itself is a simple increment counter. It allows you to increment up or down by 1 or another integer magnitude. It also allows you to limit the counting to a range.

## Build
To build this app, use `wasm-pack`. If you don't already have it installed, install it with:

```
$ cargo install wasm-pack
```

You can also find more general instructions for `wasm-pack` [here](https://yew.rs/docs/en/getting-started/project-setup/using-wasm-pack/).

```
$ wasm-pack build --target web --out-name wasm --out-dir ./static
```

## Bundle
To bundle:

```
$ rollup ./main.js --format iife --file ./pkg/bundle.js
```

## Serve
You can serve with any server of your choice. I recommend `serve` (via NPM):

```
$ npm install serve
```

Then start serving with:

```
$ serve static
```

`serve` will print out the address of the app in your terminal window.