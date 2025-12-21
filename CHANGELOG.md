# Changelog

## [Unreleased]

## [0.0.0]

### Features


- *(bindings)* Add Map type with constructor - ([b92e78d](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/b92e78d3eb434b42ce890c4d06c2415412395940))
- *(bindings)* Add MapOptions - ([717921f](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/717921feb613182b58ba8ec4b304e9fef8d63fa1))
- *(interface)* Re-export HtmlElement - ([802bbec](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/802bbece8d301e6f9a41cb7578e722576d8c13db))
- *(interface)* Add initial Map struct with constructor/MapOptions::build - ([7037e0a](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/7037e0a064236815df2a7aec2f28c5cdfecd5705))
- *(interface)* Add MapOptions struct with container/style/zoom/logo/center - ([cce4cb6](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/cce4cb6137e83b195b16854860ba9e1161c06cf8))
- *(interface)* Add MapStyleOption struct for MapOptions field - ([e91137e](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/e91137e498393b643f1e26767f798b1b6142c687))
- *(interface)* Add MapZoom struct for MapOptions field - ([71d8b6a](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/71d8b6a7e1df207295939cf1217baa47d2c9be61))
- *(interface)* Add MapContainer struct for MapOptions field - ([46d1f82](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/46d1f82abebe948c8f27b0f3bcb11d49602570e7))
- *(interface)* Add LngLatLike - ([5de2cb4](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/5de2cb4f7a995fae2787f9ac5d5a1faf261753f0))
- *(interface)* Add LngLat - ([a229fc7](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/a229fc7bc7fda4db0495897458a1804a0da2c643))

### Documentation


- Add base book structure - ([ca34097](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/ca34097f14d6226dc2ed61b878ab0d8f6dfde231))

### Testing


- Add a way to get all keys from an object - ([4409c6b](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/4409c6b04ec9c539849fd8a5313bb091d6ddb664))
- Add a function to read a value from a JsValue - ([c633350](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/c633350bfb097acfbe02a7d94e00e1bb3747168f))
- Add a way to create arbitrary html elements with automatic drop when unused - ([2a2f64b](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/2a2f64b1b0f78fedc5ec46eac8fa82cc5d46f218))
- Allow loading MapLibre GL programatically - ([858edb7](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/858edb728b35eaa7f102b368aeaa39bf93c296af))
- Mark wasm tests to always be run in browser - ([271775e](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/271775ec92bf2d383561329c0ee50927231e008f))

### Miscellaneous Tasks


- Add release-plz changelog - ([a01f5c4](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/a01f5c4d49bc40e658d20743fe9af1b3028d428c))
- Add release-plz workflow - ([0efffeb](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/0efffeb4d5e1e226f61a1d71b46b9b7a27ca5755))
- Set the homepage as the GitHub Pages site - ([8fb3b7e](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/8fb3b7ed1a117faf20707202728d6e5d01094916))
- Add initial GitHub Action - ([1752e98](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/1752e988d38de5806421fd7458a1e4028b4b8458))
- Add generated dist folder to gitignore - ([36cda56](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/36cda56d29af595577ba3c0ca8f9ba9437f57499))
- Add examples folder - ([5ab891d](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/5ab891dd792b18d3d2aea203e559204364fad1c8))
- Add map options submodule - ([5370568](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/537056885ed8759dde1408bd73d5fd7486ed2a97))
- Add interface module - ([c0f0410](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/c0f0410bf4f3f4b9f8b82bbfe1434098af884983))
- Add bindings module - ([305659c](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/305659c3658e228d499a3369a9b20c099ce02d4d))
- Add test_utils test module - ([2f58633](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/2f58633349bcfa57d1a15a532750b3873539d33e))
- Init - ([4ef6c06](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/4ef6c06a2f57f6c756859ee512c470baf636b9f7))

### Build


- Add toolchain file - ([f5d6bdd](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/f5d6bddca3dad9f301d07a9e897b0d8372abafa4))
- Add serde - ([5cf2d46](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/5cf2d46a395e4b20588520324a7c6475ab864684))
- Add serde_wasm_bindgen - ([5748705](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/5748705b5a7d3a85ce3c48360ba48b6eddfdb101))
- Add wasm_bindgen_futures to dependencies - ([d3e703a](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/d3e703a41754e3b4fb745d1343bb41cd9c62f7fe))
- Add web-sys to dependencies - ([f169da3](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/f169da39c582d1afea3be61386beaa6a2663fcc7))
- Add wasm-bindgen-test to dev-dependencies - ([922e4af](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/922e4af86babd799bea42483a24908c21ea0b3c5))
- Add wasm-bindgen to dependencies - ([0f1badd](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/0f1badd1d0551038be6890e51822325e05391200))

### Clippy


- Supress warning of futures not being send - ([66b95c3](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/66b95c3a6c0cd658fb99b67ae9acc8fdbc57a06a))

### Example


- Add Display a map with HTMLElement example - ([e99def6](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/e99def6a2ff6685123bd40691c03a73c65074390))
- Add Display a map example - ([6cd701a](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/6cd701af3f2824c12db82153f77a0e73fa26aa7f))

### Xtask


- Add book preprocessor to generate examples - ([1a7b012](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/1a7b0127dff86cfc499d6891c08aa73bdba6ba56))
- Add trunk to install dependencies - ([c1d1b71](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/c1d1b7114ce40c0966ec6bab32b2623b3db8dbd7))
- Add installdeps/format/clippy/test/wasmtest/check commands - ([0c18fb7](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/0c18fb765cde89767dd9aba763c8ef931bcfa919))
- Add pre-commit hook - ([13fa515](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/13fa5150b8f4ac08de4e2ad2f4cba3ea5c97b965))
- Add xtask crate - ([0563ed0](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/0563ed08b674a672c26cbf136371e304c85d676d))

