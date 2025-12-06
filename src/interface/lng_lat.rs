//! Rusty interface to the bindings of `Map` of MapLibre GL JS

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::bindings;

/// `LngLat` object reperesenting a given longitude and latitude, measured
/// in degrees. The coordinates are based in the WGS84 (EPSG:4326)
/// standard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LngLat {
    /// JS object that the struct wraps
    #[serde(with = "serde_wasm_bindgen::preserve")]
    js_value: bindings::LngLat,
}

impl LngLat {
    /// Create a `LngLat` object with the given latitude and longitude
    ///
    /// # Errors
    ///
    /// Propagates errors generated from JS
    pub fn new(lng: f64, lat: f64) -> Result<Self, super::Error> {
        let js_value = bindings::LngLat::new(lng, lat)?;
        Ok(Self { js_value })
    }

    /// Returns the aproximate distance between two pair of coordinates in
    /// meters.
    #[must_use]
    pub fn distance_to(&self, other: &Self) -> f64 {
        self.js_value.distance_to(&other.js_value)
    }

    /// Returns the coordinates represented as an array of two numbers.
    #[must_use]
    pub fn to_array(&self) -> [f64; 2] {
        let vec = self.js_value.to_array();
        [vec[0], vec[1]]
    }

    /// Returns a new `LagLat` whose longitude is wrapped to the range (-180,
    /// 180)
    #[must_use]
    pub fn wrap(&self) -> Self {
        Self {
            js_value: self.js_value.wrap(),
        }
    }

    /// Get the `lng` property of `LngLat`
    #[must_use]
    pub fn lng(&self) -> f64 {
        self.js_value.lng()
    }

    /// Set the `lng` property of `LngLat`
    pub fn set_lng(&mut self, lng: f64) {
        self.js_value.set_lng(lng);
    }

    /// Get the `lat` property of `LngLat`
    #[must_use]
    pub fn lat(&self) -> f64 {
        self.js_value.lat()
    }

    /// Set the `lat` property of `LngLat`
    pub fn set_lat(&mut self, lat: f64) {
        self.js_value.set_lat(lat);
    }

    /// Try converting `LngLat` into the equivalent `JsValue`
    ///
    /// # Errors
    ///
    /// Propagates `serde_wasm_bindgen` conversion errors
    pub fn as_js_value(&self) -> Result<JsValue, super::Error> {
        serde_wasm_bindgen::to_value(self)
    }

    /// Get the underlying JS object
    #[must_use]
    pub fn into_inner(self) -> bindings::LngLat {
        self.js_value
    }
}

impl std::fmt::Display for LngLat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LngLat({}, {})", self.lng(), self.lat())
    }
}

impl TryFrom<[f64; 2]> for LngLat {
    type Error = super::Error;

    fn try_from(value: [f64; 2]) -> Result<Self, Self::Error> {
        Self::new(value[0], value[1])
    }
}

impl TryFrom<(f64, f64)> for LngLat {
    type Error = super::Error;

    fn try_from(value: (f64, f64)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}

impl TryFrom<JsValue> for LngLat {
    type Error = super::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(value)
    }
}

impl TryFrom<LngLat> for JsValue {
    type Error = super::Error;

    fn try_from(value: LngLat) -> Result<Self, Self::Error> {
        value.as_js_value()
    }
}

impl TryFrom<&LngLat> for JsValue {
    type Error = super::Error;

    fn try_from(value: &LngLat) -> Result<Self, Self::Error> {
        value.as_js_value()
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::{get_value_from_object, load_maplibre_gl};

    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn new() {
        load_maplibre_gl().await;
        LngLat::new(2.3, 34.3).expect("Creating LngLat should work");
    }

    #[wasm_bindgen_test]
    async fn distance_to() {
        load_maplibre_gl().await;
        let lnglat1 = LngLat::new(0., 0.).expect("Creating LngLat should work");
        let lnglat2 = LngLat::new(0.000_009, 0.).expect("Creating LngLat should work");
        assert!(
            (lnglat1.distance_to(&lnglat2) - lnglat1.js_value.distance_to(&lnglat2.js_value)).abs()
                < 0.01
        );
    }

    #[wasm_bindgen_test]
    async fn to_array() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42)
            .expect("Creating LngLat should work")
            .to_array();

        assert!((lnglat[0] - 12.23).abs() < 0.01);
        assert!((lnglat[1] - 14.42).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn to_string() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42)
            .expect("Creating LngLat should work")
            .to_string();

        assert_eq!(lnglat.as_str(), "LngLat(12.23, 14.42)");
    }

    #[wasm_bindgen_test]
    async fn wrap() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(181., 0.)
            .expect("Creating LngLat should work")
            .wrap()
            .to_string();

        assert_eq!(lnglat.as_str(), "LngLat(-179, 0)");
    }

    #[wasm_bindgen_test]
    async fn get_lng() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        assert!((lnglat.lng() - 12.23).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn set_lng() {
        load_maplibre_gl().await;
        let mut lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        lnglat.set_lng(1.2);
        assert!((lnglat.lng() - 1.2).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn get_lat() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        assert!((lnglat.lat() - 14.42).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn set_lat() {
        load_maplibre_gl().await;
        let mut lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        lnglat.set_lat(1.2);
        assert!((lnglat.lat() - 1.2).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn try_from_tuple() {
        load_maplibre_gl().await;
        let lnglat: LngLat = (2.3, 34.3).try_into().expect("Creating LngLat should work");

        assert!((lnglat.lng() - 2.3).abs() < 0.01);
        assert!((lnglat.lat() - 34.3).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn try_from_array() {
        load_maplibre_gl().await;
        let lnglat: LngLat = [2.3, 34.3].try_into().expect("Creating LngLat should work");

        assert!((lnglat.lng() - 2.3).abs() < 0.01);
        assert!((lnglat.lat() - 34.3).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn serialization() {
        load_maplibre_gl().await;
        let lnglat: LngLat = [2.3, 34.3].try_into().expect("Creating LngLat should work");
        let lnglat_js = lnglat
            .as_js_value()
            .expect("Converting to JS Value should work");
        let retrieved_langlat_rs: LngLat = lnglat_js
            .clone()
            .try_into()
            .expect("Backconverting should work");

        assert!(
            (get_value_from_object(&lnglat_js, "lng")
                .as_f64()
                .expect("lng should be a f64")
                - 2.3)
                .abs()
                < 0.01
        );
        assert!(
            (get_value_from_object(&lnglat_js, "lat")
                .as_f64()
                .expect("lng should be a f64")
                - 34.3)
                .abs()
                < 0.01,
        );
        assert_eq!(retrieved_langlat_rs, lnglat);
    }
}
