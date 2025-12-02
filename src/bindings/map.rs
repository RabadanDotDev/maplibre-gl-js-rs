//! # Bindings to the `Map` JS object from MapLibre

use wasm_bindgen::prelude::*;

use super::MapOptions;

#[wasm_bindgen]
extern "C" {
    /// `Map` object reperesenting the map on the page.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/Map/>
    #[wasm_bindgen(js_namespace = maplibregl)]
    #[derive(Debug, Clone, PartialEq)]
    pub type Map;

    /// Create a `Map` object calling its constructor
    #[wasm_bindgen(constructor, js_namespace = maplibregl, catch)]
    pub fn new(options: MapOptions) -> Result<Map, JsValue>;

    // TODO: Add all the methods
}

#[cfg(test)]
mod test {
    use crate::test_utils::{gen_html_element_with_id, load_maplibre_gl};

    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::js_sys::{Object, Reflect};

    #[wasm_bindgen_test]
    async fn new() {
        let uuid = "fd0c7f19-d548-4fe7-aba3-5127d12b058b";
        load_maplibre_gl().await;
        let _element = gen_html_element_with_id("div", uuid);
        let options = {
            let options = Object::new();
            Reflect::set(&options, &"container".into(), &uuid.into())
                .expect("Setting container value should work");
            MapOptions::from(JsValue::from(options))
        };
        Map::new(options).expect("Creating Map should work");
    }
}
