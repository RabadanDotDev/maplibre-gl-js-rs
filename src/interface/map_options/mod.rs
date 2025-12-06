//! Rusty interface to the bindings of `MapOptions` and additional type checks of MapLibre GL JS

mod map_container;
mod map_style_option;
mod map_zoom;

pub use map_container::MapContainer;
pub use map_style_option::MapStyleOption;
pub use map_zoom::MapZoom;

use serde::Serialize;

use super::Error;
use crate::{
    bindings,
    interface::{LngLatLike, Map},
};

/// Options of a Map
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct MapOptions {
    /// The container of the map
    pub container: MapContainer,
    /// URL MapLibre style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<MapStyleOption>,
    /// Initial zoom level of the map. Defaults to the style or `0`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<MapZoom>,
    /// Whether to show the MapLibre logo
    #[serde(skip_serializing_if = "Option::is_none", rename = "maplibreLogo")]
    pub maplibre_logo: Option<bool>,
    /// Initial position of the map. Defaults to the style or the center
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<LngLatLike>,
    // TODO: Support all options
}

impl MapOptions {
    /// Create new `MapOptions` with the given container
    pub fn new(container: impl Into<MapContainer>) -> Self {
        Self {
            container: container.into(),
            style: None,
            zoom: None,
            maplibre_logo: None,
            center: None,
        }
    }

    /// Create a `Map` with the specified options
    ///
    /// # Errors
    ///
    /// Propagates errors generated on converting the given options into JS
    pub fn build(self) -> Result<Map, Error> {
        Map::new(self)
    }

    /// Specify initial zoom level of the map
    #[must_use]
    pub fn with_zoom(self, zoom: impl Into<MapZoom>) -> Self {
        Self {
            zoom: Some(zoom.into()),
            ..self
        }
    }

    /// Specify the `Map`'s style
    #[must_use]
    pub fn with_style(self, style: impl Into<MapStyleOption>) -> Self {
        Self {
            style: Some(style.into()),
            ..self
        }
    }

    /// Specify if to show the MapLibre logo in the `Map`'
    #[must_use]
    pub fn with_maplibre_logo(self) -> Self {
        Self {
            maplibre_logo: Some(true),
            ..self
        }
    }

    /// Specify the starting position of the map
    #[must_use]
    pub fn with_center(self, center: impl Into<LngLatLike>) -> Self {
        Self {
            center: Some(center.into()),
            ..self
        }
    }

    /// Try converting `MapOptions` into the equivalent bindings type
    ///
    /// # Errors
    ///
    /// Propagates `serde_wasm_bindgen` conversion errors
    pub fn as_js_value(&self) -> Result<bindings::MapOptions, super::Error> {
        Ok(bindings::MapOptions::from(serde_wasm_bindgen::to_value(
            self,
        )?))
    }

    // TODO: Add all the methods
}

impl TryFrom<MapOptions> for bindings::MapOptions {
    type Error = super::Error;

    fn try_from(value: MapOptions) -> Result<Self, super::Error> {
        value.as_js_value()
    }
}

impl TryFrom<&MapOptions> for bindings::MapOptions {
    type Error = super::Error;

    fn try_from(value: &MapOptions) -> Result<Self, Self::Error> {
        value.as_js_value()
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::{gen_html_element, get_key_list_from_object, get_value_from_object};

    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::js_sys::Boolean;

    #[wasm_bindgen_test]
    fn map_with_only_map_container_identifier_conversion() {
        let map_rust = MapOptions::new("identifier_of_map");
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_container_rs: MapContainer = get_value_from_object(&map_js, "container")
            .try_into()
            .expect("Back conversion from JS with identifier to MapContainer should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 1);
        assert_eq!(map_rust.container, retreived_container_rs);
    }

    #[wasm_bindgen_test]
    fn map_with_only_html_element_conversion() {
        let html_element = gen_html_element("div").cloned_ref();
        let map_rust = MapOptions::new(html_element);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with HTMLElement to JS should work");
        let retreived_container_rs: MapContainer = get_value_from_object(&map_js, "container")
            .try_into()
            .expect("Back conversion from JS with HTMLElement to MapContainer should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 1);
        assert_eq!(map_rust.container, retreived_container_rs);
    }

    #[wasm_bindgen_test]
    fn map_with_zoom_container_conversion() {
        let map_rust = MapOptions::new("identifier_of_map").with_zoom(3.2);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_zoom_rs: MapZoom = get_value_from_object(&map_js, "zoom")
            .try_into()
            .expect("Back conversion from JS with identifier to MapContainer should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(map_rust.zoom.unwrap(), retreived_zoom_rs);
    }

    #[wasm_bindgen_test]
    fn map_with_style_conversion() {
        let map_rust = MapOptions::new("identifier_of_map").with_style("Style URL");
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_style_rs: MapStyleOption = get_value_from_object(&map_js, "style")
            .try_into()
            .expect("Back conversion from JS with identifier to MapContainer should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(map_rust.style.unwrap(), retreived_style_rs);
    }

    #[wasm_bindgen_test]
    fn map_with_maplibre_logo() {
        let map_rust = MapOptions::new("identifier_of_map").with_maplibre_logo();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let maplibre_logo_rs: Boolean = get_value_from_object(&map_js, "maplibreLogo").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(maplibre_logo_rs.value_of());
    }

    #[wasm_bindgen_test]
    fn map_with_center() {
        let map_rust = MapOptions::new("identifier_of_map").with_center([1.2, 4.2]);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_center_rs: LngLatLike =
            bindings::LngLatLike::from(get_value_from_object(&map_js, "center"))
                .try_into()
                .expect("Back conversion from JS with identifier to MapContainer should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(map_rust.center.unwrap(), retreived_center_rs);
    }
}
