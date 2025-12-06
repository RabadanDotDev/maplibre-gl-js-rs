//! Rusty interface to the bindings of `Map` of MapLibre GL JS
use crate::{bindings, interface::MapOptions};

/// Representation of the map in the page
#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    /// JS object that the struct wraps
    js_value: bindings::Map,
}

impl Map {
    /// Create a `Map` with the specified options
    ///
    /// # Errors
    ///
    /// Propagates errors generated on converting the given options into JS
    pub fn new(options: MapOptions) -> Result<Self, super::Error> {
        let js_value = bindings::Map::new(options.try_into()?)?;
        Ok(Self { js_value })
    }

    /// Convert `Map` into the inner bindings value
    ///
    /// # Errors
    ///
    /// Propagates `serde_wasm_bindgen` conversion errors
    #[must_use]
    pub fn into_inner(self) -> bindings::Map {
        self.js_value
    }

    // TODO: support all methods
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::{gen_html_element, load_maplibre_gl};
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn map_new() {
        load_maplibre_gl().await;
        let html_element = gen_html_element("div").cloned_ref();
        MapOptions::new(html_element)
            .build()
            .expect("Creating a map should work");
    }

    #[wasm_bindgen_test]
    async fn map_new_with_style() {
        load_maplibre_gl().await;
        let html_element = gen_html_element("div").cloned_ref();
        MapOptions::new(html_element)
            .with_style("https://demotiles.maplibre.org/globe.json")
            .build()
            .expect("Creating a map should work");
    }

    #[wasm_bindgen_test]
    async fn map_new_with_style_zoom_logo_center() {
        load_maplibre_gl().await;
        let html_element = gen_html_element("div").cloned_ref();
        MapOptions::new(html_element)
            .with_style("https://demotiles.maplibre.org/globe.json")
            .with_zoom(2.)
            .with_maplibre_logo()
            .with_center((1.1, 1.1))
            .build()
            .expect("Creating a map should work");
    }
}
