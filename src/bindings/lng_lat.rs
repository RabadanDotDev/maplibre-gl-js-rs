//! # Bindings to the `LngLat` JS object from MapLibre

use wasm_bindgen::prelude::*;

use super::LngLatLike;

#[wasm_bindgen]
extern "C" {
    /// `LngLat` object reperesenting a given longitude and latitude, measured
    /// in degrees. The coordinates are based in the WGS84 (EPSG:4326)
    /// standard.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/>
    #[wasm_bindgen(js_namespace = maplibregl)]
    #[derive(Debug, Clone, PartialEq)]
    pub type LngLat;

    /// Create a `LngLat` object calling its constructor with coordinates
    /// measured in degrees
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/#constructor>
    #[wasm_bindgen(constructor, js_namespace = maplibregl, catch)]
    pub fn new(lng: f64, lat: f64) -> Result<LngLat, JsValue>;

    /// Returns the aproximate distance between two pair of coordinates in
    /// meters.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/#distanceTo>
    #[wasm_bindgen(method, js_name=distanceTo)]
    pub fn distance_to(this: &LngLat, other: &LngLat) -> f64;

    /// Returns the coordinates represented as an array of two numbers.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/#toArray>
    #[wasm_bindgen(method, js_name=toArray)]
    pub fn to_array(this: &LngLat) -> Vec<f64>;

    /// Returns the coordinates represented as a string
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/#toString>
    #[wasm_bindgen(method, js_name=toString)]
    pub fn to_string(this: &LngLat) -> String;

    /// Returns a new `LagLat` whose longitude is wrapped to the range (-180,
    /// 180)
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/#wrap>
    #[wasm_bindgen(method, js_name=wrap)]
    pub fn wrap(this: &LngLat) -> LngLat;

    /// Convert a `LngLatLike` into a `LngLat`
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/#wrap>
    #[wasm_bindgen(js_namespace = maplibregl, js_name=convert, static_method_of=LngLat, catch)]
    pub fn convert(input: &LngLatLike) -> Result<LngLat, JsValue>;

    /// Get the `lng` property of `LngLat`
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/#lng>
    #[wasm_bindgen(method, getter)]
    pub fn lng(this: &LngLat) -> f64;

    /// Set the `lng` property of `LngLat`
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/#lng>
    #[wasm_bindgen(method, setter)]
    pub fn set_lng(this: &LngLat, val: f64);

    /// Get the `lat` property of `LngLat`
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/#lat>
    #[wasm_bindgen(method, getter)]
    pub fn lat(this: &LngLat) -> f64;

    /// Set the `lat` property of `LngLat`
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/#lat>
    #[wasm_bindgen(method, setter)]
    pub fn set_lat(this: &LngLat, val: f64);
}

#[cfg(test)]
mod test {
    use crate::test_utils::load_maplibre_gl;

    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::js_sys::{Array, Object, Reflect};

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
        assert!((lnglat1.distance_to(&lnglat2).round() - 1.).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn to_array() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42)
            .expect("Creating LngLat should work")
            .to_array();

        assert_eq!(lnglat.len(), 2);
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
    async fn convert_lnglat() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        let lnglatlike = LngLatLike::from(JsValue::from(lnglat.clone()));
        assert_eq!(
            LngLat::convert(&lnglatlike)
                .expect("Conversion should work")
                .to_string(),
            lnglat.to_string()
        );
    }

    #[wasm_bindgen_test]
    async fn convert_lnglat_object() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        let lnglat_object = Object::new();
        Reflect::set(&lnglat_object, &"lng".into(), &JsValue::from(12.23))
            .expect("Setting lng should work");
        Reflect::set(&lnglat_object, &"lat".into(), &JsValue::from(14.42))
            .expect("Setting lat should work");
        let lnglatlike = LngLatLike::from(JsValue::from(lnglat_object));
        assert_eq!(
            LngLat::convert(&lnglatlike)
                .expect("Conversion should work")
                .to_string(),
            lnglat.to_string()
        );
    }

    #[wasm_bindgen_test]
    async fn convert_latlon_object() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        let lonlat_object = Object::new();
        Reflect::set(&lonlat_object, &"lon".into(), &JsValue::from(12.23))
            .expect("Setting lon should work");
        Reflect::set(&lonlat_object, &"lat".into(), &JsValue::from(14.42))
            .expect("Setting lat should work");
        let lnglatlike = LngLatLike::from(JsValue::from(lonlat_object));
        assert_eq!(
            LngLat::convert(&lnglatlike)
                .expect("Conversion should work")
                .to_string(),
            lnglat.to_string()
        );
    }

    #[wasm_bindgen_test]
    async fn convert_latlon_array() {
        load_maplibre_gl().await;
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        let lonlat_array = Array::new();
        lonlat_array.push(&JsValue::from_f64(12.23));
        lonlat_array.push(&JsValue::from_f64(14.42));
        let lnglatlike = LngLatLike::from(JsValue::from(lonlat_array));
        assert_eq!(
            LngLat::convert(&lnglatlike)
                .expect("Conversion should work")
                .to_string(),
            lnglat.to_string()
        );
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
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
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
        let lnglat = LngLat::new(12.23, 14.42).expect("Creating LngLat should work");
        lnglat.set_lat(1.2);
        assert!((lnglat.lat() - 1.2).abs() < 0.01);
    }
}
