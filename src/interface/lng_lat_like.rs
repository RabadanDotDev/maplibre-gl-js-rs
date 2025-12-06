//! Rusty interface to the bindings of `LngLatLike` of MapLibre GL JS

use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::{bindings, interface::LngLat};

/// Different options to represent a `LngLat` without necessarily having one yet
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LngLatLike {
    /// The `LngLatLike` is an actual `LngLat`
    LngLat(LngLat),
    /// The `LngLatLike` is expressed as an object
    LngLatObject {
        /// The longitude
        lng: f64,
        /// The latitude
        lat: f64,
    },
    /// The `LngLatLike` is expressed as an object with `lon` instead of `lat`
    LonLatObject {
        /// The longitude
        lon: f64,
        /// The latitude
        lat: f64,
    },
    /// The `LngLatLike` is expressed as a two index array, the first being the longitude and the
    /// second being the latitude
    Array([f64; 2]),
}

impl LngLatLike {
    /// Try converting `LngLatLike` into the equivalent bindings type
    ///
    /// # Errors
    ///
    /// Propagates `serde_wasm_bindgen` conversion errors
    pub fn as_js_value(&self) -> Result<bindings::LngLatLike, super::Error> {
        Ok(bindings::LngLatLike::from(serde_wasm_bindgen::to_value(
            self,
        )?))
    }

    /// Get the longitude of the active variant
    #[must_use]
    pub fn lng(&self) -> f64 {
        match self {
            Self::LngLat(lng_lat) => lng_lat.lng(),
            Self::LngLatObject { lng, lat: _ } | Self::Array([lng, _]) => *lng,
            Self::LonLatObject { lon, lat: _ } => *lon,
        }
    }

    /// Set the longitude of the active variant
    pub fn set_lng(&mut self, value: f64) {
        match self {
            Self::LngLat(lng_lat) => lng_lat.set_lng(value),
            Self::LngLatObject { lng, lat: _ } | Self::Array([lng, _]) => *lng = value,
            Self::LonLatObject { lon, lat: _ } => *lon = value,
        }
    }

    /// Get the latitude of the active variant
    #[must_use]
    pub fn lat(&self) -> f64 {
        match self {
            Self::LngLat(lng_lat) => lng_lat.lat(),
            Self::LngLatObject { lng: _, lat }
            | Self::LonLatObject { lon: _, lat }
            | Self::Array([_, lat]) => *lat,
        }
    }

    /// Get the latitude of the active variant
    pub fn set_lat(&mut self, value: f64) {
        match self {
            Self::LngLat(lng_lat) => lng_lat.set_lng(value),
            Self::LngLatObject { lng: _, lat }
            | Self::LonLatObject { lon: _, lat }
            | Self::Array([_, lat]) => *lat = value,
        }
    }
}

impl From<LngLat> for LngLatLike {
    fn from(value: LngLat) -> Self {
        Self::LngLat(value)
    }
}

impl From<[f64; 2]> for LngLatLike {
    fn from(value: [f64; 2]) -> Self {
        Self::Array(value)
    }
}

impl From<(f64, f64)> for LngLatLike {
    fn from(value: (f64, f64)) -> Self {
        Self::Array([value.0, value.1])
    }
}

impl TryFrom<bindings::LngLatLike> for LngLatLike {
    type Error = super::Error;

    fn try_from(value: bindings::LngLatLike) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(JsValue::from(value))
    }
}

impl TryFrom<LngLatLike> for bindings::LngLatLike {
    type Error = super::Error;

    fn try_from(value: LngLatLike) -> Result<Self, Self::Error> {
        value.as_js_value()
    }
}

impl TryFrom<&LngLatLike> for bindings::LngLatLike {
    type Error = super::Error;

    fn try_from(value: &LngLatLike) -> Result<Self, Self::Error> {
        value.as_js_value()
    }
}

impl TryFrom<LngLatLike> for LngLat {
    type Error = super::Error;

    fn try_from(value: LngLatLike) -> Result<Self, Self::Error> {
        match value {
            LngLatLike::LngLat(lng_lat) => Ok(lng_lat),
            LngLatLike::LngLatObject { lng, lat } => (lng, lat).try_into(),
            LngLatLike::LonLatObject { lon, lat } => (lon, lat).try_into(),
            LngLatLike::Array(v) => v.try_into(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::load_maplibre_gl;

    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn lnglatlike_lnglat_to_js() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        let lnglatlike = LngLatLike::LngLat(lnglat);
        let retrieved_lnglatlike: LngLatLike = lnglatlike
            .as_js_value()
            .expect("Conversion from LngLatLike with LngLat should work")
            .try_into()
            .expect("Conversion should work");

        // LngLat and LngLatObject are equally valid results for the back
        // conversion, so we just check the inner values directly
        assert!((retrieved_lnglatlike.lng() - lnglatlike.lng()) < 0.01);
        assert!((retrieved_lnglatlike.lat() - lnglatlike.lat()) < 0.01);
    }

    #[wasm_bindgen_test]
    fn lnglatlike_obj_to_js() {
        let lnglatlike = LngLatLike::LngLatObject {
            lng: 12.23,
            lat: 14.42,
        };
        let retrieved_lnglatlike: LngLatLike = lnglatlike
            .as_js_value()
            .expect("Conversion from LngLatLike with LngLat Object should work")
            .try_into()
            .expect("Conversion should work");

        assert_eq!(lnglatlike, retrieved_lnglatlike);
    }

    #[wasm_bindgen_test]
    fn lonlatlike_obj_to_js() {
        let lonlatlike = LngLatLike::LonLatObject {
            lon: 12.23,
            lat: 14.42,
        };
        let retrieved_lonlatlike: LngLatLike = lonlatlike
            .as_js_value()
            .expect("Conversion from LngLatLike with LonLat Object should work")
            .try_into()
            .expect("Conversion should work");

        assert_eq!(lonlatlike, retrieved_lonlatlike);
    }

    #[wasm_bindgen_test]
    fn lnglatlike_array_to_js() {
        let lonlatlike = LngLatLike::Array([12.23, 14.42]);
        let retrieved_lonlatlike: LngLatLike = lonlatlike
            .as_js_value()
            .expect("Conversion from LngLatLike with LonLat Array should work")
            .try_into()
            .expect("Conversion should work");

        assert_eq!(lonlatlike, retrieved_lonlatlike);
    }

    #[wasm_bindgen_test]
    async fn lnglatlike_to_lnglat() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        let lnglatlike = LngLatLike::LngLat(lnglat.clone());
        let converted_lnglat: LngLat = lnglatlike
            .try_into()
            .expect("Conversion to LngLat should work");

        assert_eq!(converted_lnglat.to_string(), lnglat.to_string());
    }

    #[wasm_bindgen_test]
    async fn lnglatlike_obj_to_lnglat() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        let lnglatlike = LngLatLike::LngLatObject {
            lng: 12.23,
            lat: 14.42,
        };
        let converted_lnglat: LngLat = lnglatlike
            .try_into()
            .expect("Conversion to LngLat should work");

        assert_eq!(converted_lnglat.to_string(), lnglat.to_string());
    }

    #[wasm_bindgen_test]
    async fn lonlatlike_obj_to_lnglat() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        let lnglatlike = LngLatLike::LonLatObject {
            lon: 12.23,
            lat: 14.42,
        };
        let converted_lnglat: LngLat = lnglatlike
            .try_into()
            .expect("Conversion to LngLat should work");

        assert_eq!(converted_lnglat.to_string(), lnglat.to_string());
    }

    #[wasm_bindgen_test]
    async fn lnglatlike_array_to_lnglat() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        let lnglatlike = LngLatLike::Array([12.23, 14.42]);
        let converted_lnglat: LngLat = lnglatlike
            .try_into()
            .expect("Conversion to LngLat should work");

        assert_eq!(converted_lnglat.to_string(), lnglat.to_string());
    }

    #[wasm_bindgen_test]
    fn from_tuple() {
        assert_eq!(
            Into::<LngLatLike>::into((12.23, 14.42)),
            LngLatLike::Array([12.23, 14.42])
        );
    }

    #[wasm_bindgen_test]
    fn from_array() {
        assert_eq!(
            Into::<LngLatLike>::into([12.23, 14.42]),
            LngLatLike::Array([12.23, 14.42])
        );
    }
}
