//! Submodule for the `MapContainer` field of `MapOptions` and associated tests

use serde::{Deserialize, Deserializer, Serialize, de::Error};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

/// Container for a map. It may be expressed as a `String` identifier or as a `HTMLElement`
/// reference. The target should be empty
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
#[serde(untagged)]
pub enum MapContainer {
    /// Container reference expressed as the `String` identifier
    Identifier(String),
    /// Container reference expressed as the `HTMLElement` itself
    #[serde(with = "serde_wasm_bindgen::preserve")]
    HtmlElement(HtmlElement),
}

// The default Deserialize implementation is unnable to properly identify 'HtmlElement'.
// therefore we need to implement the variant differentiation manually
impl<'de> Deserialize<'de> for MapContainer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val: JsValue = serde_wasm_bindgen::preserve::deserialize(deserializer)?;

        if let Some(v) = val.as_string() {
            return Ok(Self::Identifier(v));
        }

        if let Ok(v) = val.dyn_into::<HtmlElement>() {
            return Ok(Self::HtmlElement(v));
        }

        Err(D::Error::custom("Unsupported JS value for MapContainer"))
    }
}

impl MapContainer {
    /// Try converting `MapContainer` into the equivalent `JsValue`
    ///
    /// # Errors
    ///
    /// Propagates `serde_wasm_bindgen` conversion errors
    pub fn as_js_value(&self) -> Result<JsValue, super::Error> {
        serde_wasm_bindgen::to_value(self)
    }
}

impl From<&str> for MapContainer {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for MapContainer {
    fn from(value: String) -> Self {
        Self::Identifier(value)
    }
}

impl From<HtmlElement> for MapContainer {
    fn from(value: HtmlElement) -> Self {
        Self::HtmlElement(value)
    }
}

impl TryFrom<JsValue> for MapContainer {
    type Error = super::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(value)
    }
}

impl TryFrom<MapContainer> for JsValue {
    type Error = super::Error;

    fn try_from(value: MapContainer) -> Result<Self, Self::Error> {
        value.as_js_value()
    }
}

impl TryFrom<&MapContainer> for JsValue {
    type Error = super::Error;

    fn try_from(value: &MapContainer) -> Result<Self, Self::Error> {
        value.as_js_value()
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::gen_html_element;

    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn map_container_identifier_conversion() {
        let container: MapContainer = "identifier_of_map".into();
        let container_js = container
            .as_js_value()
            .expect("Conversion from MapContainer with id to JS should work");
        let retrieved_container_rs: MapContainer = container_js
            .clone()
            .try_into()
            .expect("Back conversion from JS with id to MapContainer should work");

        assert_eq!(
            container_js
                .as_string()
                .expect("Converted should be a string"),
            "identifier_of_map"
        );
        assert_eq!(container, retrieved_container_rs);
    }

    #[wasm_bindgen_test]
    fn map_container_html_element_serialization() {
        let html_element = gen_html_element("div").cloned_ref();
        let container: MapContainer = html_element.clone().into();
        let container_js = container
            .as_js_value()
            .expect("Conversion from MapContainer with HTMLElement to JS should work");

        let retrieved_container_rs: MapContainer = container_js
            .clone()
            .try_into()
            .expect("Back conversion from JS with HTMLElement to MapContainer should work");

        assert_eq!(
            container_js
                .dyn_into::<HtmlElement>()
                .expect("Converted should be an HtmlElement"),
            html_element
        );
        assert_eq!(container, retrieved_container_rs);
    }
}
