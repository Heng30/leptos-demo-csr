A demo project of full-stack development by using leptos, trunk and tailwindcss

#### Setup environment
- add wasm toolchain: `rustup target add wasm32-unknown-unknown`
- install `trunk` and `wasm-bindgen-cli`: `cargo install trunk wasm-bindgen-cli`
- install `tailwindcss`: `npm install -g tailwindcss`

#### Start web serve
- trunk serve

#### Build release
- trunk build --release
