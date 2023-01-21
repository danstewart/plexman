# Architecture

### Structure

Individual pages can be found in `src/pages/` with the entrypoint being `src/index.html`.  
Pages can either be standalone or be composed of various "frames" which can be found in `src/frames/`.

The "frames" are loaded using JavaScript (specifically using [binder](https://binder.danstewart.dev/)).

All functions exported from the rust side can found in `src-tauri/src/main.rs` inside the `main` function.

We then tie this up on the frontend in `src/js/fn.js`.

### Running

To run the app in development mode run `./tools/serve.sh`, this will install the `live-server` npm package for hot reloading and then run the `tauri dev` command.
