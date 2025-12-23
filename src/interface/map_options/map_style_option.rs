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
    /// JSON representing a non-validated MapLibre `StyleSpecification`
    JsonStyleSpecification(serde_json::Value),
    // TODO: Support validated style spec
}

impl MapStyleOption {
    /// Try converting `MapStyle` into the equivalent `JsValue`
    ///
    /// # Errors
    ///
    /// Propagates `serde_wasm_bindgen` conversion errors
    pub fn as_js_value(&self) -> Result<JsValue, super::Error> {
        self.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
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

impl From<serde_json::Value> for MapStyleOption {
    fn from(value: serde_json::Value) -> Self {
        Self::JsonStyleSpecification(value)
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
    use crate::test_utils::{get_key_list_from_object, get_value_from_object};

    use super::*;
    use serde_json::json;
    use wasm_bindgen_test::*;
    use web_sys::js_sys::Array;

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

    #[wasm_bindgen_test]
    #[allow(clippy::cast_possible_truncation)]
    fn map_style_json_spec_conversion() {
        let value = json!({
            "version": 8,
            "sources": {
                "satellite": {
                    "type": "raster",
                    "tiles": [
                        "https://tiles.maps.eox.at/wmts/1.0.0/s2cloudless-2020_3857/default/g/{z}/{y}/{x}.jpg"
                    ],
                    "tileSize": 256
                }
            },
            "layers": [{
                "id": "satellite",
                "type": "raster",
                "source": "satellite"
            }]
        });
        let map_style: MapStyleOption = value.into();
        let map_style_js = map_style
            .as_js_value()
            .expect("Should be able to convert to JsValue");
        let map_style_retrieved = MapStyleOption::try_from(map_style_js.clone())
            .expect("Should be able to back convert map style");

        assert_eq!(map_style, map_style_retrieved);

        let fields = get_key_list_from_object(&map_style_js);
        let version = get_value_from_object(&map_style_js, "version");
        let sources = get_value_from_object(&map_style_js, "sources");
        let layers = get_value_from_object(&map_style_js, "layers");
        let sources_keys = get_key_list_from_object(&sources);

        assert_eq!(fields.len(), 3);
        assert_eq!(version.as_f64().unwrap() as i32, 8);
        assert_eq!(sources_keys, vec!["satellite"]);
        assert_eq!(Array::from(&layers).to_vec().len(), 1);
    }
}
