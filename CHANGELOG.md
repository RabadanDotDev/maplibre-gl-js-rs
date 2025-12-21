# Changelog

## [Unreleased]

## [0.0.0]

### Features


- *(bindings)* Add Map type with constructor - ([4657975](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/465797503d872d9118ddd14cbb89366f265fe3fe))
- *(bindings)* Add MapOptions - ([717921f](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/717921feb613182b58ba8ec4b304e9fef8d63fa1))
- *(interface)* Re-export HtmlElement - ([d21184b](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/d21184b950db50c6e0e41aa0462e6d8837a95f49))
- *(interface)* Add initial Map struct with constructor/MapOptions::build - ([8f1bc76](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/8f1bc76de94f62cb299a51fb7069a2fe546827de))
- *(interface)* Add MapOptions struct with container/style/zoom/logo/center - ([a34f79c](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/a34f79cb613c08f4bb76d2d727f1ff2e81998d04))
- *(interface)* Add MapStyleOption struct for MapOptions field - ([18e4cbe](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/18e4cbe06f34d17f287bbcc61ff95d5b5b8c988b))
- *(interface)* Add MapZoom struct for MapOptions field - ([891bc9c](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/891bc9c5330493d174626a6560529a22378bfafb))
- *(interface)* Add MapContainer struct for MapOptions field - ([12833c5](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/12833c52f7784370d6b39622137794d536f72ed7))
- *(interface)* Add LngLatLike - ([070048f](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/070048fca44f78e0dce259eb078625cbb00b0b94))
- *(interface)* Add LngLat - ([d60a3ff](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/d60a3ff52ecb0fe7027a73594dcc59604f6d1761))

### Documentation


- Add base book structure - ([6dc9908](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/6dc990877a4d166e5c3364a824f06d18d302cafc))

### Testing


- Add a way to get all keys from an object - ([dbbe4fd](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/dbbe4fd5251782e126746b66e6dfceae73a8f986))
- Add a function to read a value from a JsValue - ([c633350](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/c633350bfb097acfbe02a7d94e00e1bb3747168f))
- Add a way to create arbitrary html elements with automatic drop when unused - ([2a2f64b](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/2a2f64b1b0f78fedc5ec46eac8fa82cc5d46f218))
- Allow loading MapLibre GL programatically - ([858edb7](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/858edb728b35eaa7f102b368aeaa39bf93c296af))
- Mark wasm tests to always be run in browser - ([271775e](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/271775ec92bf2d383561329c0ee50927231e008f))

### Miscellaneous Tasks


- Add release-plz changelog - ([29976c5](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/29976c5b35af314a3c5f6cb735811df91d308c62))
- Add release-plz workflow - ([22e06d5](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/22e06d5c6e37b5995290a5a9df5f3b1d4e2ca16a))
- Set the homepage as the GitHub Pages site - ([340f9c5](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/340f9c5212dcc662ffa732f1e5bc8463e3e4c083))
- Add initial GitHub Action - ([306e7e7](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/306e7e7b3c263f381bfbe82a55f32fd372217399))
- Add generated dist folder to gitignore - ([da467e2](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/da467e24f583880bc900c4f33f8b7afae67c37b7))
- Add examples folder - ([07c33c1](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/07c33c1223972073bdd2f063e47d964a2ac72e84))
- Add map options submodule - ([f2475c2](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/f2475c23a59af71344210bbbe97cec1a9e015618))
- Add interface module - ([271fbc5](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/271fbc5d0692f0d8845aa558dc5dee1a99fa5b46))
- Add bindings module - ([305659c](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/305659c3658e228d499a3369a9b20c099ce02d4d))
- Add test_utils test module - ([2f58633](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/2f58633349bcfa57d1a15a532750b3873539d33e))
- Init - ([4ef6c06](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/4ef6c06a2f57f6c756859ee512c470baf636b9f7))

### Build


- Add toolchain file - ([b900f77](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/b900f7713750ef424a9f7afccd1cbc6f40ff5c91))
- Add serde - ([5cf2d46](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/5cf2d46a395e4b20588520324a7c6475ab864684))
- Add serde_wasm_bindgen - ([5748705](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/5748705b5a7d3a85ce3c48360ba48b6eddfdb101))
- Add wasm_bindgen_futures to dependencies - ([d3e703a](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/d3e703a41754e3b4fb745d1343bb41cd9c62f7fe))
- Add web-sys to dependencies - ([f169da3](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/f169da39c582d1afea3be61386beaa6a2663fcc7))
- Add wasm-bindgen-test to dev-dependencies - ([922e4af](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/922e4af86babd799bea42483a24908c21ea0b3c5))
- Add wasm-bindgen to dependencies - ([0f1badd](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/0f1badd1d0551038be6890e51822325e05391200))

### Clippy


- Supress warning of futures not being send - ([66b95c3](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/66b95c3a6c0cd658fb99b67ae9acc8fdbc57a06a))

### Example


- Add Display a map with HTMLElement example - ([2f5f04f](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/2f5f04fec5168b77655b1d67f89b908223af2516))
- Add Display a map example - ([4d959e4](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/4d959e446eb8848f53ba40395d868d23dfa5dfd7))

### Xtask


- Add book preprocessor to generate examples - ([1ff30eb](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/1ff30eb41b467f4e99dbfc9d63468c4935530eb0))
- Add trunk to install dependencies - ([b162c8c](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/b162c8c0c2c0e7f03beeb0f6f1998fd80fdcc29c))
- Add installdeps/format/clippy/test/wasmtest/check commands - ([0c18fb7](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/0c18fb765cde89767dd9aba763c8ef931bcfa919))
- Add pre-commit hook - ([13fa515](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/13fa5150b8f4ac08de4e2ad2f4cba3ea5c97b965))
- Add xtask crate - ([0563ed0](https://github.com/RabadanDotDev/maplibre-gl-js-rs/commit/0563ed08b674a672c26cbf136371e304c85d676d))

