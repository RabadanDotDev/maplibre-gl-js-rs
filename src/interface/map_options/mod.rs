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
        name: attribution_control,
        serde_rename: "attributionControl",
        inner_type: (), // TODO: create type
        type_docs: "Attribution control configuration",
        implement: nothing,
        implement_docs: "Specify the initial bearing of the map",
    },
    {
        name: bearing,
        serde_rename: "bearing",
        inner_type: f64,
        type_docs: "Initial bearing (rotation) of the map in counter-clockwise degrees from north. \
                    Defaults to the style or to `0`",
        implement: set_type,
        implement_docs: "Specify the initial bearing of the map",
    },
    {
        name: bearing_snap,
        serde_rename: "bearingSnap",
        inner_type: f64,
        type_docs: "Threshold to snap the map's bearing to north. Defaults to `7`",
        implement: set_type,
        implement_docs: "Specify the bearing snap threshold of the map",
    },
    {
        name: bounds,
        serde_rename: "bounds",
        inner_type: (), // TODO: create type
        type_docs: "Initial bounds of the map. Overrides `center` and `zoom`",
        implement: nothing,
        implement_docs: "Specify the initial bounds of the map. Overrides `center` and `zoom`",
    },
    {
        name: box_zoom_interaction,
        serde_rename: "boxZoom",
        inner_type: bool,
        type_docs: "Whether the 'box zoom' interaction is enabled. Defaults to `true`",
        implement: set_false,
        implement_docs: "Disable the 'box zoom' interaction",
    },
    {
        name: cancel_pending_tile_requests_while_zooming,
        serde_rename: "cancelPendingTileRequestsWhileZooming",
        inner_type: bool,
        type_docs: "Whether to cancel the loading of 'in-progress' tiles if the zoom in level has \
                    changed. Activating this is more efficient, but may have abrupt changes. \
                    Defaults to `true`",
        implement: set_false,
        implement_docs: "Disable the cancelation of the load of pending tiles. Smother experience \
                         but less efficient",
    },
    {
        name: canvas_context_attributes,
        serde_rename: "canvasContextAttributes",
        inner_type: (), // TODO: create type
        type_docs: "`WebGLContextAttributes` to apply to the `WebGL` context of the map",
        implement: nothing,
        implement_docs: "Specify the initial bounds of the map. Overrides `center` and `zoom`",
    },
    {
        name: center,
        serde_rename: "center",
        inner_type: LngLatLike,
        type_docs: "Initial position of the map. Defaults to the style or to 0,0",
        implement: set_type,
        implement_docs: "Specify the starting position of the map",
    },
    {
        name: center_clamped_to_ground,
        serde_rename: "centerClampedToGround",
        inner_type: bool,
        type_docs: "Whether to automatically set the elevation of the center point to the terrain \
                    elevation or default to sea level and not update. Needs to be set to false to \
                    keep the camera above ground when pitch > 90 degrees",
        implement: set_false,
        implement_docs: "Disable automatically setting the elevation of the center point to the \
                         terrain elevation and leave the defaulted sea level value. This may need \
                         to be disabled to keep the camera above ground then the pich is > 90 \
                         degrees",
    },
    {
        name: click_tolerance,
        serde_rename: "clickTolerance",
        inner_type: u32,
        type_docs: "Max number of pixels that the cursor can move before considering a click a \
                    drag. Defaults to `3`",
        implement: set_type,
        implement_docs: "Specify the max number of pixels that the cursor can move before \
                         considering a click a drag",
    },
    {
        name: collect_resource_timing,
        serde_rename: "collectResourceTiming",
        inner_type: bool,
        type_docs: "Whether to collect Resource Timing API information and include it as \
                    `resouceTiming` in relevant `data` events. Defaults to `false`",
        implement: set_true,
        implement_docs: "Collect Resource Timing API information and include it as \
                         `resouceTiming` in relevant `data` events.",
    },
    {
        name: cooperative_gestures,
        serde_rename: "cooperativeGestures",
        inner_type: bool,
        type_docs: "Whether to activate cooperative gestures. Defaults to `false`",
        implement: set_true,
        implement_docs: "Enable coopeative gestures",
    },
    {
        name: cross_source_collisions,
        serde_rename: "crossSourceCollisions",
        inner_type: bool,
        type_docs: "Whether symbols from multiple sources can collide. Defaults to `true`",
        implement: set_false,
        implement_docs: "Disable symbol collision check for symbols from different sources",
    },
    {
        name: double_click_zoom,
        serde_rename: "doubleClickZoom",
        inner_type: bool,
        type_docs: "Whether 'double click to zoom' interaction is enabled. Defaults to `true`",
        implement: set_false,
        implement_docs: "Disable 'double click to zoom' interaction",
    },
    {
        name: drag_pan,
        serde_rename: "dragPan",
        inner_type: (), // TODO: create type
        type_docs: "Whether 'drag to pan' interaction is enabled with options. Defaults to `true`",
        implement: nothing,
        implement_docs: "",
    },
    {
        name: drag_rotate,
        serde_rename: "dragRotate",
        inner_type: bool,
        type_docs: "Whether 'drag to rotate' interaction is enabled. Defaults to `true`",
        implement: set_false,
        implement_docs: "Disable 'drag to rotate' interaction",
    },
    {
        name: elevation,
        serde_rename: "elevation",
        inner_type: f64,
        type_docs: "The elevation of the initial geographical centerpoint of the map, in meters  \
                    above sea level. Defaults to `0`",
        implement: set_type,
        implement_docs: "Specify the elevation of the initial geographical centerpoint of the \
                         map, in meters above sea level.",
    },
    {
        name: experimental_zoom_levels_to_overscale,
        serde_rename: "experimentalZoomLevelsToOverscale",
        inner_type: f64,
        type_docs: "Amount of overzoom levels to allow after max zoom by splitting previous vector \
                    tiles",
        implement: set_type,
        implement_docs: "Set the amount of overzoom levels to allow after max zoom by splitting \
                         previous vector tiles",
    },
    {
        name: fade_duration,
        serde_rename: "fadeDuration",
        inner_type: f64,
        type_docs: "Duration in milliseconds for the fade-in/out animation for label collitions \
                    after initial map load. Defaults to `300`",
        implement: set_type,
        implement_docs: "Set the duration in milliseconds for the fade-in/out animation for label \
                         collisions after initial map load.",
    },
    {
        name: fit_bounds_options,
        serde_rename: "fitBoundsOptions",
        inner_type: (), // TODO: create type
        type_docs: "Additional options to use when specifying `bounds`",
        implement: nothing,
        implement_docs: "Set additional options to use when specifying `bounds`",
    },
    {
        name: hash,
        serde_rename: "hash",
        inner_type: (), // TODO: create type
        type_docs: "Whether to sync the map position with the fragment of the page's URL and \
                    optionally how. Defaults to `false`",
        implement: nothing,
        implement_docs: "Enable syncing the map position with the fragment of the page's URL and \
                         optionally how",
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
        name: keyboard,
        serde_rename: "keyboard",
        inner_type: bool,
        type_docs: "Whether the keyboard shortcuts are enabled. Defaults to `true`",
        implement: set_false,
        implement_docs: "Disable keyboard shortcuts",
    },
    {
        name: locale,
        serde_rename: "locale",
        inner_type: (), // TODO: create type
        type_docs: "Patch to apply to the default localization table for UI strings",
        implement: nothing,
        implement_docs: "",
    },
    {
        name: local_ideograph_font_family,
        serde_rename: "localIdeographFontFamily",
        inner_type: (), // TODO: create type
        type_docs: "CSS font-family for locally overriding generation of CJK characters. Defaults \
                    to `sans-serif`",
        implement: nothing,
        implement_docs: "",
    },
    {
        name: logo_position,
        serde_rename: "logoPosition",
        inner_type: (), // TODO: create type
        type_docs: "Where to put the MapLibre wordmark on the map. Defaults to `bottom-left`",
        implement: nothing,
        implement_docs: "Set the position for the MapLibre wordmark",
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
        name: max_bounds,
        serde_rename: "maxBounds",
        inner_type: (), // TODO: create type
        type_docs: "Whether to constrain the map bounds and with which",
        implement: nothing,
        implement_docs: "Set map bounds",
    },
    {
        name: max_canvas_size,
        serde_rename: "maxCanvasSize",
        inner_type: [u32; 2],
        type_docs: "The `width` and `height` max size of the canvas. Defaults to `[4096, 4096]`",
        implement: set_type,
        implement_docs: "The `width` and `height` max size of the canvas. Should be lower than \
                         WebGl MAX_TEXTURE_SIZE ",
    },
    {
        name: max_pitch,
        serde_rename: "maxPitch",
        inner_type: f64,
        type_docs: "The maximum pitch of the map (0-180). Defaults to `60`",
        implement: set_type,
        implement_docs: "Specify the the maximum pitch of the map (0-180)",
    },
    {
        name: max_tile_cache_size,
        serde_rename: "maxTileCacheSize",
        inner_type: u32,
        type_docs: "The maximum number of cached tiles for a given source. Dynamically set if \
                    unspecified",
        implement: set_type,
        implement_docs: "Set the maximum number of cached tiles for a given source.",
    },
    {
        name: max_tile_cache_zoom_levels,
        serde_rename: "maxTileCacheZoomLevels",
        inner_type: u32,
        type_docs: "The maximum number of zoom levels to cache tiles for a given source. Defaults \
                    to 5",
        implement: set_type,
        implement_docs: "Set the maximum number of zoom levels to cache tiles for a given source.",
    },
    {
        name: max_zoom,
        serde_rename: "maxZoom",
        inner_type: f64,
        type_docs: "The maximum zoom of the map (0-24). Defaults to `22`",
        implement: set_type,
        implement_docs: "Specify the the maximum zoom of the map (0-24)",
    },
    {
        name: min_pitch,
        serde_rename: "minPitch",
        inner_type: f64,
        type_docs: "The minimum pitch of the map (0-180). Defaults to `0`",
        implement: set_type,
        implement_docs: "Specify the the minimum pitch of the map (0-180)",
    },
    {
        name: min_zoom,
        serde_rename: "minZoom",
        inner_type: f64,
        type_docs: "The minimum zoom of the map (0-24). Defaults to `0`",
        implement: set_type,
        implement_docs: "Specify the the minimum zoom of the map (0-24)",
    },
    {
        name: pitch,
        serde_rename: "pitch",
        inner_type: f64,
        type_docs: "The initial tilt of the map, in degrees away from the plane of the screen. \
                    Defaults to the style or `0`",
        implement: set_type,
        implement_docs: "The initial tilt of the map, in degrees away from the plane of the screen.",
    },
    {
        name: pitch_with_rotate,
        serde_rename: "pitchWithRotate",
        inner_type: bool,
        type_docs: "Whether the map pitch/tilt with 'drag to rotate' interaction is enabled. \
                    Defaults to `true`",
        implement: set_false,
        implement_docs: "Disable the map pitch/tilt with 'drag to rotate' interaction",
    },
    {
        name: pixel_ratio,
        serde_rename: "pixelRatio",
        inner_type: f64,
        type_docs: "The pixel ratio between the width/height of the container and the canvas \
                    Defaults to `devicePixelRatio`",
        implement: set_type,
        implement_docs: "The initial tilt of the map, in degrees away from the plane of the screen.",
    },
    {
        name: reduce_motion,
        serde_rename: "reduceMotion",
        inner_type: bool,
        type_docs: "Whether to disable inertia or not. Defaults to the user device settings",
        implement: set_true,
        implement_docs: "Disable gesture inertia",
    },
    {
        name: refresh_expired_tiles,
        serde_rename: "refreshExpiredTiles",
        inner_type: bool,
        type_docs: "Whether to re-request expired tiles per HTTP cache headers \
                    Defaults to `true`",
        implement: set_false,
        implement_docs: "Disable the re-requesting of expired tiles",
    },
    {
        name: render_world_copies,
        serde_rename: "renderWorldCopies",
        inner_type: bool,
        type_docs: "Whether to render 'copies' of the map if zoomed out enough. Defaults to \
                    `true`",
        implement: set_false,
        implement_docs: "Disable the rendering of 'copies' if the map is zoomed out enough",
    },
    {
        name: roll,
        serde_rename: "roll",
        inner_type: f64,
        type_docs: "The initial roll angle of the map, in degrees counter-clockwise about the \
                    camera boresight. Defaults to the style or `0`",
        implement: set_type,
        implement_docs: "The initial roll angle of the map, in degrees counter-clockwise about \
                         the camera boresight.",
    },
    {
        name: roll_enabled,
        serde_rename: "rollEnabled",
        inner_type: bool,
        type_docs: "Whether to enable 'drag to rotate' interaction for the map roll. Defaults to \
                    `false`",
        implement: set_true,
        implement_docs: "Enable the 'drag to rotate' interaction for the map's roll",
    },
    {
        name: scroll_zoom,
        serde_rename: "scrollZoom",
        inner_type: (), // TODO: create type
        type_docs: "Whether 'scroll to zoom' interaction is enabled with options. Defaults to `true`",
        implement: nothing,
        implement_docs: "",
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
        name: touch_pitch,
        serde_rename: "touchPitch",
        inner_type: (), // TODO: create type
        type_docs: "Whether 'drag to pitch' interaction is enabled with options. Defaults to `true`",
        implement: nothing,
        implement_docs: "",
    },
    {
        name: touch_zoom_rotate,
        serde_rename: "touchZoomRotate",
        inner_type: (), // TODO: create type
        type_docs: "Whether 'pinch to rotate and zoom' interaction is enabled with options. Defaults to `true`",
        implement: nothing,
        implement_docs: "",
    },
    {
        name: track_resize,
        serde_rename: "trackResize",
        inner_type: bool,
        type_docs: "Whether to automatically reaize the map when the window resizes. Defaults to \
                    `true`",
        implement: set_false,
        implement_docs: "Disable the automatic map resize when the window resizes",
    },
    {
        name: transform_camera_update,
        serde_rename: "transformCameraUpdate",
        inner_type: (), // TODO: create type
        type_docs: "Callback run before the map's camera is moved",
        implement: nothing,
        implement_docs: "",
    },
    {
        name: transform_constrain,
        serde_rename: "transformConstrain",
        inner_type: (), // TODO: create type
        type_docs: "Callback to override how the map constrains the viewport's lnglat and zoom to \
                    respect the longlat bounds",
        implement: nothing,
        implement_docs: "",
    },
    {
        name: transform_request,
        serde_rename: "transformRequest",
        inner_type: (), // TODO: create type
        type_docs: "Callback run before the Map makes a request for an external URL",
        implement: nothing,
        implement_docs: "",
    },
    {
        name: validate_style,
        serde_rename: "validateStyle",
        inner_type: bool,
        type_docs: "Whether to run style validation. Defaults to `true`",
        implement: set_false,
        implement_docs: "Disable style validation",
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
    use web_sys::js_sys::{Array, Boolean};

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
    fn map_with_bearing() {
        let map_rust = MapOptions::new("identifier_of_map").with_bearing(140);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "bearing")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.bearing.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    fn map_with_bearing_snap() {
        let map_rust = MapOptions::new("identifier_of_map").with_bearing_snap(5);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "bearingSnap")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.bearing_snap.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    fn map_without_box_zoom_interaction() {
        let map_rust = MapOptions::new("identifier_of_map").without_box_zoom_interaction();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "boxZoom").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
    }

    #[wasm_bindgen_test]
    fn map_without_cancel_pending_tile_requests_while_zooming() {
        let map_rust = MapOptions::new("identifier_of_map")
            .without_cancel_pending_tile_requests_while_zooming();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean =
            get_value_from_object(&map_js, "cancelPendingTileRequestsWhileZooming").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
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
    fn map_without_center_clamped_to_ground() {
        let map_rust = MapOptions::new("identifier_of_map").without_center_clamped_to_ground();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "centerClampedToGround").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
    }

    #[wasm_bindgen_test]
    #[allow(clippy::cast_possible_truncation)]
    fn map_with_click_tolerance() {
        let map_rust = MapOptions::new("identifier_of_map").with_click_tolerance(4u32);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs = get_value_from_object(&map_js, "clickTolerance")
            .as_f64()
            .expect("Should be a valid number") as i32;
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, 4);
    }

    #[wasm_bindgen_test]
    fn map_with_collect_resource_timing() {
        let map_rust = MapOptions::new("identifier_of_map").with_collect_resource_timing();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "collectResourceTiming").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, true);
    }

    #[wasm_bindgen_test]
    fn map_with_cooperative_gestures() {
        let map_rust = MapOptions::new("identifier_of_map").with_cooperative_gestures();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "cooperativeGestures").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, true);
    }

    #[wasm_bindgen_test]
    fn map_without_cross_source_collisions() {
        let map_rust = MapOptions::new("identifier_of_map").without_cross_source_collisions();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "crossSourceCollisions").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
    }

    #[wasm_bindgen_test]
    fn map_without_double_click_zoom() {
        let map_rust = MapOptions::new("identifier_of_map").without_double_click_zoom();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "doubleClickZoom").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
    }

    #[wasm_bindgen_test]
    fn map_without_drag_rotate() {
        let map_rust = MapOptions::new("identifier_of_map").without_drag_rotate();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "dragRotate").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
    }

    #[wasm_bindgen_test]
    fn map_with_elevation() {
        let map_rust = MapOptions::new("identifier_of_map").with_elevation(10);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "elevation")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.elevation.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    fn map_with_experimental_zoom_levels_to_overscale() {
        let map_rust =
            MapOptions::new("identifier_of_map").with_experimental_zoom_levels_to_overscale(5);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "experimentalZoomLevelsToOverscale")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.experimental_zoom_levels_to_overscale.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    fn map_with_fade_duration() {
        let map_rust = MapOptions::new("identifier_of_map").with_fade_duration(1234);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "fadeDuration")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.fade_duration.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    fn map_without_interactivity() {
        let map_rust = MapOptions::new("identifier_of_map").without_interactivity();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "interactive").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
    }

    #[wasm_bindgen_test]
    fn map_without_keyboard() {
        let map_rust = MapOptions::new("identifier_of_map").without_keyboard();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "keyboard").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
    }

    #[wasm_bindgen_test]
    fn map_with_maplibre_logo() {
        let map_rust = MapOptions::new("identifier_of_map").with_maplibre_logo();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "maplibreLogo").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, true);
    }

    #[wasm_bindgen_test]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn map_with_max_canvas_size() {
        let map_rust = MapOptions::new("identifier_of_map").with_max_canvas_size([1234, 4321]);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs = Array::from(&get_value_from_object(&map_js, "maxCanvasSize")).to_vec();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs.len(), 2);
        assert_eq!(retreived_rs[0].as_f64().unwrap() as u32, 1234);
        assert_eq!(retreived_rs[1].as_f64().unwrap() as u32, 4321);
    }

    #[wasm_bindgen_test]
    fn map_with_max_pitch() {
        let map_rust = MapOptions::new("identifier_of_map").with_max_pitch(40);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "maxPitch")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.max_pitch.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn map_with_max_tile_cache_size() {
        let map_rust = MapOptions::new("identifier_of_map").with_max_tile_cache_size(234u32);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs = get_value_from_object(&map_js, "maxTileCacheSize")
            .as_f64()
            .expect("Back conversion from JS with identifier should work")
            as u32;
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, 234);
    }

    #[wasm_bindgen_test]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn map_with_max_tile_zoom_levels() {
        let map_rust = MapOptions::new("identifier_of_map").with_max_tile_cache_zoom_levels(7u32);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs = get_value_from_object(&map_js, "maxTileCacheZoomLevels")
            .as_f64()
            .expect("Back conversion from JS with identifier should work")
            as u32;
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, 7);
    }

    #[wasm_bindgen_test]
    fn map_with_max_zoom() {
        let map_rust = MapOptions::new("identifier_of_map").with_max_zoom(8);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "maxZoom")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.max_zoom.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    fn map_with_min_pitch() {
        let map_rust = MapOptions::new("identifier_of_map").with_min_pitch(40);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "minPitch")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.min_pitch.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    fn map_with_min_zoom() {
        let map_rust = MapOptions::new("identifier_of_map").with_min_zoom(8);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "minZoom")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.min_zoom.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    fn map_with_pitch() {
        let map_rust = MapOptions::new("identifier_of_map").with_pitch(40);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "pitch")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.pitch.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    fn map_without_pitch_with_rotate() {
        let map_rust = MapOptions::new("identifier_of_map").without_pitch_with_rotate();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "pitchWithRotate").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
    }

    #[wasm_bindgen_test]
    fn map_with_pixel_ratio() {
        let map_rust = MapOptions::new("identifier_of_map").with_pixel_ratio(1.3);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "pixelRatio")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.pixel_ratio.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    fn map_with_reduce_motion() {
        let map_rust = MapOptions::new("identifier_of_map").with_reduce_motion();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "reduceMotion").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, true);
    }

    #[wasm_bindgen_test]
    fn map_without_refresh_expired_tiles() {
        let map_rust = MapOptions::new("identifier_of_map").without_refresh_expired_tiles();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "refreshExpiredTiles").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
    }

    #[wasm_bindgen_test]
    fn map_without_render_world_copies() {
        let map_rust = MapOptions::new("identifier_of_map").without_render_world_copies();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "renderWorldCopies").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
    }

    #[wasm_bindgen_test]
    fn map_with_roll() {
        let map_rust = MapOptions::new("identifier_of_map").with_roll(40);
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: f64 = get_value_from_object(&map_js, "roll")
            .try_into()
            .expect("Back conversion from JS with identifier should work");
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert!(map_rust.roll.unwrap() - retreived_rs < 0.001);
    }

    #[wasm_bindgen_test]
    fn map_with_roll_enabled() {
        let map_rust = MapOptions::new("identifier_of_map").with_roll_enabled();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "rollEnabled").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, true);
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

    #[wasm_bindgen_test]
    fn map_without_track_resize() {
        let map_rust = MapOptions::new("identifier_of_map").without_track_resize();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "trackResize").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
    }

    #[wasm_bindgen_test]
    fn map_without_validate_style() {
        let map_rust = MapOptions::new("identifier_of_map").without_validate_style();
        let map_js = map_rust
            .as_js_value()
            .expect("Conversion from MapContainer with identifier to JS should work");
        let retreived_rs: Boolean = get_value_from_object(&map_js, "validateStyle").into();
        let keys = get_key_list_from_object(&map_js);

        assert_eq!(keys.len(), 2);
        assert_eq!(retreived_rs, false);
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
}
