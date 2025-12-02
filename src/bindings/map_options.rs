//! # Bindings to the `MapOptions` JS object from MapLibre

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// The options for `Map`
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/type-aliases/MapOptions/>
    #[wasm_bindgen(js_namespace = maplibregl)]
    #[derive(Debug, Clone, PartialEq)]
    pub type MapOptions;
}
