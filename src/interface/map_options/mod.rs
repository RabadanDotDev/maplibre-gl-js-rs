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

/// Generate `MapOptions` struct with builder functions
macro_rules! declare_map_options {
    (
        $(
            {
                name: $name:ident,
                serde_rename: $serde_rename:expr,
                inner_type: $inner_type:ty,
                type_docs: $type_docs:expr,
                implement: $implement:ident,
                implement_docs: $implement_docs:expr,
            }
        ),* $(,)?
    ) => {
        /// Options of a map
        #[derive(Debug, PartialEq, Clone, Serialize)]
        pub struct MapOptions {
            /// The container of the map
            container: MapContainer,
            $(
                #[doc = $type_docs]
                #[serde(
                    skip_serializing_if = "Option::is_none",
                    rename = $serde_rename
                )]
                $name: Option<$inner_type>,
            )*
        }

        impl MapOptions {
            /// Create new `MapOptions` with the given container
            pub fn new(container: impl Into<MapContainer>) -> Self {
                Self {
                    container: container.into(),
                    $(
                        $name: None,
                    )*
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

            /// Try converting `MapOptions` into the equivalent bindings type
            ///
            /// # Errors
            ///
            /// Propagates `serde_wasm_bindgen` conversion errors
            pub fn as_js_value(&self) -> Result<bindings::MapOptions, super::Error> {
                Ok(bindings::MapOptions::from(self.serialize(
                    &serde_wasm_bindgen::Serializer::json_compatible(),
                )?))
            }

            $(
                declare_map_options!(@impl $implement, $name, $inner_type, $implement_docs);
            )*
        }
    };

    (@impl nothing, $name:ident, $inner_type:ty, $implement_docs:expr) => {
    };
    (@impl set_type, $name:ident, $inner_type:ty, $implement_docs:expr) => {
        paste::paste! {
            #[doc = $implement_docs]
            #[must_use]
            pub fn [<with_ $name>](self, $name: impl Into<$inner_type>) -> Self {
                Self {
                    $name: Some($name.into()),
                    ..self
                }
            }
        }
    };
    (@impl set_true, $name:ident, $inner_type:ty, $implement_docs:expr) => {
        paste::paste! {
            #[doc = $implement_docs]
            #[must_use]
            pub fn [<with_ $name>](self) -> Self {
                Self {
                    $name: Some(true),
                    ..self
                }
            }
        }
    };
    (@impl set_false, $name:ident, $inner_type:ty, $implement_docs:expr) => {
        paste::paste! {
            #[doc = $implement_docs]
            #[must_use]
            pub fn [<without_ $name>](self) -> Self {
                Self {
                    $name: Some(false),
                    ..self
                }
            }
        }
    };
}

declare_map_options!(
    {
        name: center,
        serde_rename: "center",
        inner_type: LngLatLike,
        type_docs: "Initial position of the map. Defaults to the style or to 0,0",
        implement: set_type,
        implement_docs: "Specify the starting position of the map",
    },
    {
        name: interactivity,
        serde_rename: "interactive",
        inner_type: bool,
        type_docs: "Whether the map is interactive or not. Defaults to `true`",
        implement: set_false,
        implement_docs: "Mark the map as not interactive",
    },
    {
        name: maplibre_logo,
        serde_rename: "maplibreLogo",
        inner_type: bool,
        type_docs: "Whether to show the MapLibre logo",
        implement: set_true,
        implement_docs: "Specify if to show the MapLibre logo in the `Map`",
    },
    {
        name: style,
        serde_rename: "style",
        inner_type: MapStyleOption,
        type_docs: "MapLibre style",
        implement: set_type,
        implement_docs: "Specify the `Map`'s style",
    },
    {
        name: zoom,
        serde_rename: "zoom",
        inner_type: MapZoom,
        type_docs: "Initial zoom level of the map. Defaults to the style or `0`",
        implement: set_type,
        implement_docs: "Specify initial zoom level of the map",
    },
);

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
    use serde_json::json;
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

    #[wasm_bindgen_test]
    fn map_without_interactivity() {
        let map_rust = MapOptions::new("identifier_of_map").without_interactivity();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let maplibre_interactive_rs: Boolean = get_value_from_object(&map_js, "interactive").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(!maplibre_interactive_rs.value_of());
    }

    #[wasm_bindgen_test]
    fn map_with_json_style() {
        let map_rust = MapOptions::new("identifier_of_map").with_style(json!({
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
        }));
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let map_style_js = get_value_from_object(&map_js, "style");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(get_key_list_from_object(&map_style_js).len(), 3);
    }
}
