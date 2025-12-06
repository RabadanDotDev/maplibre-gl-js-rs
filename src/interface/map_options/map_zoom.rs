//! Submodule for the `MapZoom` field of `MapOptions` and associated tests

use std::ops::Deref;

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

/// Zoom level of the map
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct MapZoom(f64);

impl MapZoom {
    /// Try converting `MapZoom` into the equivalent `JsValue`
    ///
    /// # Errors
    ///
    /// Propagates `serde_wasm_bindgen` conversion errors
    pub fn as_js_value(self) -> Result<JsValue, super::Error> {
        serde_wasm_bindgen::to_value(&self)
    }
}

impl From<f64> for MapZoom {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl TryFrom<JsValue> for MapZoom {
    type Error = super::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(value)
    }
}

impl TryFrom<MapZoom> for JsValue {
    type Error = super::Error;

    fn try_from(value: MapZoom) -> Result<Self, Self::Error> {
        value.as_js_value()
    }
}

impl Deref for MapZoom {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn map_zoom_conversion() {
        let map_zoom = MapZoom(32.0);
        let map_zoom_js = map_zoom
            .as_js_value()
            .expect("Should be able to convert to JsValue");
        let map_zoom_retrieved = MapZoom::try_from(map_zoom_js.clone())
            .expect("Should be able to back convert map zoom");

        assert!((map_zoom_js.as_f64().expect("Converted should be a f64") - 32.0) < 0.01);
        assert!((*map_zoom_retrieved - 32.0) < 0.01);
    }
}
