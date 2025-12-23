//! Submodule for the `MapStyleOption` field of `MapOptions` and associated tests

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

/// The map style
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum MapStyleOption {
    /// URL to a JSON object following the MapLibre Style Specification
    URL(String),
    // TODO: Support direct style specification
}

impl MapStyleOption {
    /// Try converting `MapStyle` into the equivalent `JsValue`
    ///
    /// # Errors
    ///
    /// Propagates `serde_wasm_bindgen` conversion errors
    pub fn as_js_value(&self) -> Result<JsValue, super::Error> {
        serde_wasm_bindgen::to_value(self)
    }
}

impl From<&str> for MapStyleOption {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for MapStyleOption {
    fn from(value: String) -> Self {
        Self::URL(value)
    }
}

impl TryFrom<JsValue> for MapStyleOption {
    type Error = super::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(value)
    }
}

impl TryFrom<MapStyleOption> for JsValue {
    type Error = super::Error;

    fn try_from(value: MapStyleOption) -> Result<Self, Self::Error> {
        value.as_js_value()
    }
}

impl TryFrom<&MapStyleOption> for JsValue {
    type Error = super::Error;

    fn try_from(value: &MapStyleOption) -> Result<Self, Self::Error> {
        value.as_js_value()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn map_style_url_conversion() {
        let map_style: MapStyleOption = "url".into();
        let map_style_js = map_style
            .as_js_value()
            .expect("Should be able to convert to JsValue");
        let map_style_retrieved = MapStyleOption::try_from(map_style_js.clone())
            .expect("Should be able to back convert map style");

        assert_eq!(
            map_style_js
                .as_string()
                .expect("Converted should be a string"),
            "url"
        );
        assert_eq!(map_style, map_style_retrieved);
    }
}
