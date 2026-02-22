//! # Bindings to the `LngLatBoundsLike` JS object from MapLibre

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// LngLatBoundsLike`` object reperesenting something that can be conveted into
    /// an actual `LngLatBounds`
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/type-aliases/LngLatBoundsLike/>
    #[wasm_bindgen(js_namespace = maplibregl)]
    #[derive(Debug, Clone, PartialEq)]
    pub type LngLatBoundsLike;
}
