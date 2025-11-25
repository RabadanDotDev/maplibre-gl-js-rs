//! # Bindings to the `LngLatLike` JS object from MapLibre

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// `LngLatLike` object reperesenting something that can be conveted into
    /// an actual `LngLat`
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/type-aliases/LngLatLike/>
    #[wasm_bindgen(js_namespace = maplibregl)]
    #[derive(Debug, Clone, PartialEq)]
    pub type LngLatLike;
}
