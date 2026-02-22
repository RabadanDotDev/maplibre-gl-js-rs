//! # Bindings to the `LngLat` JS object from MapLibre

use wasm_bindgen::prelude::*;
use web_sys::js_sys::Array;

use super::LngLat;
use super::LngLatBoundsLike;
use super::LngLatLike;

#[wasm_bindgen]
extern "C" {
    /// `LngLatBounds` object reperesenting a given longitude and latitude, measured
    /// in degrees. The coordinates are based in the WGS84 (EPSG:4326)
    /// standard.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLat/>
    #[wasm_bindgen(js_namespace = maplibregl)]
    #[derive(Debug, Clone, PartialEq)]
    pub type LngLatBounds;

    /// Create null `LngLatBounds`
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#constructor>
    #[wasm_bindgen(constructor, js_namespace = maplibregl)]
    pub fn null() -> LngLatBounds;

    /// Create a `LngLatBounds` from the coordinates of the four sides.
    ///
    /// The array should be 4 number elements ordered by:
    ///
    /// - west
    /// - south
    /// - east
    /// - north
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#constructor>
    #[wasm_bindgen(constructor, js_namespace = maplibregl)]
    pub fn from_corner_values(sw: &Array) -> LngLatBounds;

    /// Create a `LngLatBounds` from the two main corners with an array
    ///
    /// The array should be 2 LngLatLike elements ordered by:
    ///
    /// - south/west
    /// - north/east
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#constructor>
    #[wasm_bindgen(constructor, js_namespace = maplibregl)]
    pub fn from_corner_vec(corners: &Array) -> LngLatBounds;

    /// Create a `LngLatBounds` from the two main corners
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#constructor>
    #[wasm_bindgen(constructor, js_namespace = maplibregl)]
    pub fn from_corner(south_west: &LngLatLike, north_east: &LngLatLike) -> LngLatBounds;

    /// Adjusts the given bounds to handle the case where the bounds cross the 180th meridian (antimeridian).
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#adjustantimeridian>
    #[wasm_bindgen(method, js_name=adjustAntiMeridian)]
    pub fn adjust_anti_meridian(this: &LngLatBounds) -> LngLatBounds;

    /// Check if the point is within the bounding box.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#contains>
    #[wasm_bindgen(method, js_name=contains)]
    pub fn contains(this: &LngLatBounds, lng_lat: &LngLatLike) -> bool;

    /// Extend the bounds to include a given LngLatLike.
    ///
    /// WARNING: The returned value is `this`. wasm-bindgen does not permit capturing ownersip or
    /// marking it as mutable
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#extend>
    #[wasm_bindgen(method, js_name=extend)]
    pub fn extend_with_lng_lat_like(this: &LngLatBounds, lng_lat_like: &LngLatLike)
    -> LngLatBounds;

    /// Extend the bounds to include a given LngLatLike.
    ///
    /// WARNING: The returned value is `this`. wasm-bindgen does not permit capturing ownersip or
    /// marking it as mutable
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#extend>
    #[wasm_bindgen(method, js_name=extend)]
    pub fn extend_with_lng_lat_bounds_like(
        this: &LngLatBounds,
        lng_lat_bounds_like: &LngLatBoundsLike,
    ) -> LngLatBounds;

    /// Returns the east edge of the bounding box.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#geteast>
    #[wasm_bindgen(method, js_name=getEast)]
    pub fn get_east(this: &LngLatBounds) -> f64;

    /// Returns the north edge of the bounding box.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#getnorth>
    #[wasm_bindgen(method, js_name=getNorth)]
    pub fn get_north(this: &LngLatBounds) -> f64;

    /// Returns the northeast corner of the bounding box.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#getnortheast>
    #[wasm_bindgen(method, js_name=getNorthEast)]
    pub fn get_north_east(this: &LngLatBounds) -> LngLat;

    /// Returns the northwest corner of the bounding box.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#getnorthwest>
    #[wasm_bindgen(method, js_name=getNorthWest)]
    pub fn get_north_west(this: &LngLatBounds) -> LngLat;

    /// Returns the south edge of the bounding box.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#getsouth>
    #[wasm_bindgen(method, js_name=getSouth)]
    pub fn get_south(this: &LngLatBounds) -> f64;

    /// Returns the southeast corner of the bounding box.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#getsoutheast>
    #[wasm_bindgen(method, js_name=getSouthEast)]
    pub fn get_south_east(this: &LngLatBounds) -> LngLat;

    /// Returns the southwest corner of the bounding box.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#getsouthwest>
    #[wasm_bindgen(method, js_name=getSouthWest)]
    pub fn get_south_west(this: &LngLatBounds) -> LngLat;

    /// Returns the west edge of the bounding box.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#getwest>
    #[wasm_bindgen(method, js_name=getWest)]
    pub fn get_west(this: &LngLatBounds) -> f64;

    /// Checks if this bounding box intersects with another bounding box.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#intersects>
    #[wasm_bindgen(method, js_name=intersects)]
    pub fn intersects(this: &LngLatBounds, other: &LngLatBoundsLike) -> bool;

    /// Check if the bounding box is an empty/null-type box.
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#isempty>
    #[wasm_bindgen(method, js_name=isEmpty)]
    pub fn is_empty(this: &LngLatBounds) -> bool;

    /// Set the northeast corner of the bounding box
    ///
    /// WARNING: The returned value is `this`. wasm-bindgen does not permit capturing ownersip or
    /// marking it as mutable
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#setnortheast>
    #[wasm_bindgen(method, js_name=setNorthEast)]
    pub fn set_north_east(this: &LngLatBounds, north_east: &LngLatLike) -> LngLatBounds;

    /// Set the southwest corner of the bounding box
    ///
    /// WARNING: The returned value is `this`. wasm-bindgen does not permit capturing ownersip or
    /// marking it as mutable
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#setsouthwest>
    #[wasm_bindgen(method, js_name=setSouthWest)]
    pub fn set_south_west(this: &LngLatBounds, south_west: &LngLatLike) -> LngLatBounds;

    /// Returns the bounding box represented as an array of ararys
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#toarray>
    #[wasm_bindgen(method, js_name=toArray)]
    pub fn to_array(this: &LngLatBounds) -> Vec<Array>;

    /// Returns the bounding box represented as a string
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#tostring>
    #[wasm_bindgen(method, js_name=toString)]
    pub fn to_string(this: &LngLatBounds) -> String;

    /// Convert a `LngLatBoundsLike` Into a `LngLatBounds`
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#convert>
    #[wasm_bindgen(js_namespace = maplibregl, js_name=convert, static_method_of=LngLatBounds)]
    pub fn convert(input: &LngLatBoundsLike) -> LngLatBounds;

    /// Create a `LngLatBounds` from a LngLat center and a radius
    ///
    /// MapLibre docs: <https://maplibre.org/maplibre-gl-js/docs/API/classes/LngLatBounds/#fromLngLat>
    #[wasm_bindgen(js_namespace = maplibregl, js_name=fromLngLat, static_method_of=LngLatBounds)]
    pub fn from_lng_lat(center: &LngLat, radius: f64) -> LngLatBounds;
}

#[cfg(test)]
mod test {
    use crate::test_utils::load_maplibre_gl;

    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::js_sys::Array;

    #[wasm_bindgen_test]
    async fn from_corner_values() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0.2, 0.4, 0.6, 0.8]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        assert!((lng_lat_bounds.get_west() - 0.2).abs() < 0.01);
        assert!((lng_lat_bounds.get_south() - 0.4).abs() < 0.01);
        assert!((lng_lat_bounds.get_east() - 0.6).abs() < 0.01);
        assert!((lng_lat_bounds.get_north() - 0.8).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn from_corner_vec() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![
            LngLat::new(0.2, 0.4).expect("LngLat construction should work"),
            LngLat::new(0.6, 0.8).expect("LngLat construction should work"),
        ]));
        let lng_lat_bounds = LngLatBounds::from_corner_vec(&bounds);
        assert!((lng_lat_bounds.get_west() - 0.2).abs() < 0.01);
        assert!((lng_lat_bounds.get_south() - 0.4).abs() < 0.01);
        assert!((lng_lat_bounds.get_east() - 0.6).abs() < 0.01);
        assert!((lng_lat_bounds.get_north() - 0.8).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn from_corner() {
        load_maplibre_gl().await;
        let bounds = [0.2, 0.4, 0.6, 0.8];
        let lng_lat_bounds = LngLatBounds::from_corner(
            &LngLatLike::unchecked_from_js(LngLat::new(bounds[0], bounds[1]).unwrap().into()),
            &LngLatLike::unchecked_from_js(LngLat::new(bounds[2], bounds[3]).unwrap().into()),
        );
        assert!((lng_lat_bounds.get_west() - 0.2).abs() < 0.01);
        assert!((lng_lat_bounds.get_south() - 0.4).abs() < 0.01);
        assert!((lng_lat_bounds.get_east() - 0.6).abs() < 0.01);
        assert!((lng_lat_bounds.get_north() - 0.8).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn adjust_anti_meridian() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![
            175.813_127,
            -20.157_768,
            -178.340_903,
            -15.449_124,
        ]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        let lng_lat_bounds = lng_lat_bounds.adjust_anti_meridian();
        assert!((lng_lat_bounds.get_west() - 175.813_127).abs() < 0.000_001);
        assert!((lng_lat_bounds.get_south() - -20.157_768).abs() < 0.000_001);
        assert!((lng_lat_bounds.get_east() - 181.659_097).abs() < 0.000_001);
        assert!((lng_lat_bounds.get_north() - -15.449_124).abs() < 0.000_001);
    }

    #[wasm_bindgen_test]
    async fn contains() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0., 0., 10., 10.]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        let point1 = LngLatLike::unchecked_from_js(LngLat::new(5., 5.).unwrap().into());
        let point2 = LngLatLike::unchecked_from_js(LngLat::new(11., 11.).unwrap().into());
        assert!(lng_lat_bounds.contains(&point1));
        assert!(!lng_lat_bounds.contains(&point2));
    }

    #[wasm_bindgen_test]
    async fn extend_with_lng_lat_like_inside() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0., 0., 10., 10.]));
        let point = LngLatLike::unchecked_from_js(LngLat::new(5., 5.).unwrap().into());

        let lng_lat_bounds_og = LngLatBounds::from_corner_values(&bounds);
        let lng_lat_bounds_exp = LngLatBounds::from_corner_values(&bounds);

        let lng_lat_bounds_tr = lng_lat_bounds_og.extend_with_lng_lat_like(&point);

        assert!((lng_lat_bounds_tr.get_west() - lng_lat_bounds_exp.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_south() - lng_lat_bounds_exp.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_east() - lng_lat_bounds_exp.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_north() - lng_lat_bounds_exp.get_north()).abs() < 0.01);

        assert!((lng_lat_bounds_exp.get_west() - lng_lat_bounds_og.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_south() - lng_lat_bounds_og.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_east() - lng_lat_bounds_og.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_north() - lng_lat_bounds_og.get_north()).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn extend_with_lng_lat_like_outside() {
        load_maplibre_gl().await;
        let bounds_og = Array::from(&JsValue::from(vec![0., 0., 10., 10.]));
        let bounds_ext = Array::from(&JsValue::from(vec![0., -5., 15., 10.]));
        let point = LngLatLike::unchecked_from_js(LngLat::new(15., -5.).unwrap().into());

        let lng_lat_bounds_og = LngLatBounds::from_corner_values(&bounds_og);
        let lng_lat_bounds_exp = LngLatBounds::from_corner_values(&bounds_ext);
        let lng_lat_bounds_tr = lng_lat_bounds_og.extend_with_lng_lat_like(&point);

        assert!((lng_lat_bounds_tr.get_west() - lng_lat_bounds_exp.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_south() - lng_lat_bounds_exp.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_east() - lng_lat_bounds_exp.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_north() - lng_lat_bounds_exp.get_north()).abs() < 0.01);

        assert!((lng_lat_bounds_exp.get_west() - lng_lat_bounds_og.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_south() - lng_lat_bounds_og.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_east() - lng_lat_bounds_og.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_north() - lng_lat_bounds_og.get_north()).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn extend_with_lng_lat_bounds_like_inside() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0., 0., 10., 10.]));
        let bounds_in = Array::from(&JsValue::from(vec![3., 3., 7., 7.]));
        let lng_lat_bounds_in = LngLatBoundsLike::unchecked_from_js(
            LngLatBounds::from_corner_values(&bounds_in).into(),
        );

        let lng_lat_bounds_og = LngLatBounds::from_corner_values(&bounds);
        let lng_lat_bounds_exp = LngLatBounds::from_corner_values(&bounds);
        let lng_lat_bounds_tr =
            lng_lat_bounds_og.extend_with_lng_lat_bounds_like(&lng_lat_bounds_in);

        assert!((lng_lat_bounds_tr.get_west() - lng_lat_bounds_exp.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_south() - lng_lat_bounds_exp.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_east() - lng_lat_bounds_exp.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_north() - lng_lat_bounds_exp.get_north()).abs() < 0.01);

        assert!((lng_lat_bounds_exp.get_west() - lng_lat_bounds_og.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_south() - lng_lat_bounds_og.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_east() - lng_lat_bounds_og.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_north() - lng_lat_bounds_og.get_north()).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn extend_with_lng_lat_bounds_like_outside() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0., 0., 10., 10.]));
        let bounds_in = Array::from(&JsValue::from(vec![5., 5., 15., 15.]));
        let bounds_exp = Array::from(&JsValue::from(vec![0., 0., 15., 15.]));
        let lng_lat_bounds_in = LngLatBoundsLike::unchecked_from_js(
            LngLatBounds::from_corner_values(&bounds_in).into(),
        );

        let lng_lat_bounds_og = LngLatBounds::from_corner_values(&bounds);
        let lng_lat_bounds_exp = LngLatBounds::from_corner_values(&bounds_exp);
        let lng_lat_bounds_tr =
            lng_lat_bounds_og.extend_with_lng_lat_bounds_like(&lng_lat_bounds_in);

        assert!((lng_lat_bounds_tr.get_west() - lng_lat_bounds_exp.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_south() - lng_lat_bounds_exp.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_east() - lng_lat_bounds_exp.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_north() - lng_lat_bounds_exp.get_north()).abs() < 0.01);

        assert!((lng_lat_bounds_exp.get_west() - lng_lat_bounds_og.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_south() - lng_lat_bounds_og.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_east() - lng_lat_bounds_og.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_north() - lng_lat_bounds_og.get_north()).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn get_east() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0., 0., 0.123_456, 0.]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        assert!((lng_lat_bounds.get_east() - 0.123_456).abs() < 0.000_001);
    }

    #[wasm_bindgen_test]
    async fn get_north() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0., 0., 0., 0.123_456]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        assert!((lng_lat_bounds.get_north() - 0.123_456).abs() < 0.000_001);
    }

    #[wasm_bindgen_test]
    async fn get_north_east() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0., 0., 0.789_012, 0.123_456]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        let lng_lat = lng_lat_bounds.get_north_east();
        assert!((lng_lat.lng() - 0.789_012).abs() < 0.000_001);
        assert!((lng_lat.lat() - 0.123_456).abs() < 0.000_001);
    }

    #[wasm_bindgen_test]
    async fn get_north_west() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0.789_012, 0., 0., 0.123_456]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        let lng_lat = lng_lat_bounds.get_north_west();
        assert!((lng_lat.lng() - 0.789_012).abs() < 0.000_001);
        assert!((lng_lat.lat() - 0.123_456).abs() < 0.000_001);
    }

    #[wasm_bindgen_test]
    async fn get_south() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0., 0.123_456, 0., 0.]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        assert!((lng_lat_bounds.get_south() - 0.123_456).abs() < 0.000_001);
    }

    #[wasm_bindgen_test]
    async fn get_south_east() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0., 0.123_456, 0.789_012, 0.]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        let lng_lat = lng_lat_bounds.get_south_east();
        assert!((lng_lat.lng() - 0.789_012).abs() < 0.000_001);
        assert!((lng_lat.lat() - 0.123_456).abs() < 0.000_001);
    }

    #[wasm_bindgen_test]
    async fn get_south_west() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0.789_012, 0.123_456, 0., 0.]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        let lng_lat = lng_lat_bounds.get_south_west();
        assert!((lng_lat.lng() - 0.789_012).abs() < 0.000_001);
        assert!((lng_lat.lat() - 0.123_456).abs() < 0.000_001);
    }

    #[wasm_bindgen_test]
    async fn get_west() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0.123_456, 0., 0., 0.]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        assert!((lng_lat_bounds.get_west() - 0.123_456).abs() < 0.000_001);
    }

    #[wasm_bindgen_test]
    async fn intersects_full_inner() {
        load_maplibre_gl().await;
        let bounds1 = Array::from(&JsValue::from(vec![0., 0., 10., 10.]));
        let bounds2 = Array::from(&JsValue::from(vec![3., 3., 7., 7.]));

        let lng_lat_bounds1 = LngLatBounds::from_corner_values(&bounds1);
        let lng_lat_bounds2 =
            LngLatBoundsLike::unchecked_from_js(LngLatBounds::from_corner_values(&bounds2).into());

        assert!(lng_lat_bounds1.intersects(&lng_lat_bounds2));
    }

    #[wasm_bindgen_test]
    async fn intersects_part_inner() {
        load_maplibre_gl().await;
        let bounds1 = Array::from(&JsValue::from(vec![0., 0., 10., 10.]));
        let bounds2 = Array::from(&JsValue::from(vec![3., 3., 15., 15.]));

        let lng_lat_bounds1 = LngLatBounds::from_corner_values(&bounds1);
        let lng_lat_bounds2 =
            LngLatBoundsLike::unchecked_from_js(LngLatBounds::from_corner_values(&bounds2).into());

        assert!(lng_lat_bounds1.intersects(&lng_lat_bounds2));
    }

    #[wasm_bindgen_test]
    async fn intersects_boundary() {
        load_maplibre_gl().await;
        let bounds1 = Array::from(&JsValue::from(vec![0., 0., 10., 10.]));
        let bounds2 = Array::from(&JsValue::from(vec![10., 10., 15., 15.]));

        let lng_lat_bounds1 = LngLatBounds::from_corner_values(&bounds1);
        let lng_lat_bounds2 =
            LngLatBoundsLike::unchecked_from_js(LngLatBounds::from_corner_values(&bounds2).into());

        assert!(lng_lat_bounds1.intersects(&lng_lat_bounds2));
    }

    #[wasm_bindgen_test]
    async fn intersects_part_outer() {
        load_maplibre_gl().await;
        let bounds1 = Array::from(&JsValue::from(vec![0., 0., 10., 10.]));
        let bounds2 = Array::from(&JsValue::from(vec![11., 11., 15., 15.]));

        let lng_lat_bounds1 = LngLatBounds::from_corner_values(&bounds1);
        let lng_lat_bounds2 =
            LngLatBoundsLike::unchecked_from_js(LngLatBounds::from_corner_values(&bounds2).into());

        assert!(!lng_lat_bounds1.intersects(&lng_lat_bounds2));
    }

    #[wasm_bindgen_test]
    async fn is_empty_non_empty() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0., 0., 10., 10.]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);

        assert!(!lng_lat_bounds.is_empty());
    }

    #[wasm_bindgen_test]
    async fn is_empty_non_empty_smallest() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![0., 0., 0., 0.]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);

        assert!(!lng_lat_bounds.is_empty());
    }

    #[wasm_bindgen_test]
    async fn null_is_empty() {
        load_maplibre_gl().await;
        let lng_lat_bounds = LngLatBounds::null();

        assert!(lng_lat_bounds.is_empty());
    }

    #[wasm_bindgen_test]
    async fn set_north_east() {
        load_maplibre_gl().await;
        let bounds_exp = Array::from(&JsValue::from(vec![0., 0., 0.789_012, 0.123_456]));
        let bounds_og = Array::from(&JsValue::from(vec![0., 0., 0., 0.]));
        let point =
            LngLatLike::unchecked_from_js(LngLat::new(0.789_012, 0.123_456).unwrap().into());

        let lng_lat_bounds_exp = LngLatBounds::from_corner_values(&bounds_exp);
        let lng_lat_bounds_og = LngLatBounds::from_corner_values(&bounds_og);
        let lng_lat_bounds_tr = lng_lat_bounds_og.set_north_east(&point);

        assert!((lng_lat_bounds_tr.get_west() - lng_lat_bounds_exp.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_south() - lng_lat_bounds_exp.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_east() - lng_lat_bounds_exp.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_north() - lng_lat_bounds_exp.get_north()).abs() < 0.01);

        assert!((lng_lat_bounds_exp.get_west() - lng_lat_bounds_og.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_south() - lng_lat_bounds_og.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_east() - lng_lat_bounds_og.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_north() - lng_lat_bounds_og.get_north()).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn set_south_west() {
        load_maplibre_gl().await;
        let bounds_exp = Array::from(&JsValue::from(vec![0.789_012, 0.123_456, 0., 0.]));
        let bounds_og = Array::from(&JsValue::from(vec![0., 0., 0., 0.]));
        let point1 =
            LngLatLike::unchecked_from_js(LngLat::new(0.789_012, 0.123_456).unwrap().into());

        let lng_lat_bounds_exp = LngLatBounds::from_corner_values(&bounds_exp);
        let lng_lat_bounds_og = LngLatBounds::from_corner_values(&bounds_og);
        let lng_lat_bounds_tr = lng_lat_bounds_og.set_south_west(&point1);

        assert!((lng_lat_bounds_tr.get_west() - lng_lat_bounds_exp.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_south() - lng_lat_bounds_exp.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_east() - lng_lat_bounds_exp.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_tr.get_north() - lng_lat_bounds_exp.get_north()).abs() < 0.01);

        assert!((lng_lat_bounds_exp.get_west() - lng_lat_bounds_og.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_south() - lng_lat_bounds_og.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_east() - lng_lat_bounds_og.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_north() - lng_lat_bounds_og.get_north()).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn to_array() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![1., 2., 3., 4.]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);
        let array = lng_lat_bounds.to_array();

        assert_eq!(array.len(), 2);
        assert_eq!(array[0].length(), 2);
        assert_eq!(array[1].length(), 2);
        assert!((array[0].get(0).as_f64().unwrap() - 1.).abs() < 0.01);
        assert!((array[0].get(1).as_f64().unwrap() - 2.).abs() < 0.01);
        assert!((array[1].get(0).as_f64().unwrap() - 3.).abs() < 0.01);
        assert!((array[1].get(1).as_f64().unwrap() - 4.).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn to_string() {
        load_maplibre_gl().await;
        let bounds = Array::from(&JsValue::from(vec![1., 2., 3., 4.]));
        let lng_lat_bounds = LngLatBounds::from_corner_values(&bounds);

        assert_eq!(
            lng_lat_bounds.to_string(),
            "LngLatBounds(LngLat(1, 2), LngLat(3, 4))".to_string()
        );
    }

    #[wasm_bindgen_test]
    async fn convert_from_lnglatbounds() {
        load_maplibre_gl().await;
        let bounds_exp = Array::from(&JsValue::from(vec![1., 2., 3., 4.]));
        let lng_lat_bounds_exp = LngLatBounds::from_corner_values(&bounds_exp);

        let bounds_in = Array::from(&JsValue::from(vec![1., 2., 3., 4.]));
        let lng_lat_bounds_in = LngLatBoundsLike::unchecked_from_js(
            LngLatBounds::from_corner_values(&bounds_in).into(),
        );
        let lng_lat_bounds = LngLatBounds::convert(&lng_lat_bounds_in);

        assert!((lng_lat_bounds_exp.get_west() - lng_lat_bounds.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_south() - lng_lat_bounds.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_east() - lng_lat_bounds.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_north() - lng_lat_bounds.get_north()).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn convert_from_lnglatlike_pair() {
        load_maplibre_gl().await;
        let bounds_exp = Array::from(&JsValue::from(vec![1., 2., 3., 4.]));
        let lng_lat_bounds_exp = LngLatBounds::from_corner_values(&bounds_exp);

        let bounds_in = LngLatBoundsLike::unchecked_from_js(
            Array::from(&JsValue::from(vec![
                LngLat::new(1., 2.).expect("LngLat construction should work"),
                LngLat::new(3., 4.).expect("LngLat construction should work"),
            ]))
            .into(),
        );
        let lng_lat_bounds = LngLatBounds::convert(&bounds_in);

        assert!((lng_lat_bounds_exp.get_west() - lng_lat_bounds.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_south() - lng_lat_bounds.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_east() - lng_lat_bounds.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_north() - lng_lat_bounds.get_north()).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn convert_from_corners() {
        load_maplibre_gl().await;
        let bounds_exp = Array::from(&JsValue::from(vec![1., 2., 3., 4.]));
        let lng_lat_bounds_exp = LngLatBounds::from_corner_values(&bounds_exp);

        let bounds_in = LngLatBoundsLike::unchecked_from_js(
            Array::from(&JsValue::from(vec![1., 2., 3., 4.])).into(),
        );
        let lng_lat_bounds = LngLatBounds::convert(&bounds_in);

        assert!((lng_lat_bounds_exp.get_west() - lng_lat_bounds.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_south() - lng_lat_bounds.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_east() - lng_lat_bounds.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_north() - lng_lat_bounds.get_north()).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn from_lng_lat_rad0() {
        load_maplibre_gl().await;
        let bounds_exp = Array::from(&JsValue::from(vec![0., 0., 0., 0.]));
        let lng_lat_bounds_exp = LngLatBounds::from_corner_values(&bounds_exp);

        let lng_lat = LngLat::new(0., 0.).unwrap();
        let lng_lat_bounds = LngLatBounds::from_lng_lat(&lng_lat, 0.);

        assert!((lng_lat_bounds_exp.get_west() - lng_lat_bounds.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_south() - lng_lat_bounds.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_east() - lng_lat_bounds.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_north() - lng_lat_bounds.get_north()).abs() < 0.01);
    }

    #[wasm_bindgen_test]
    async fn from_lng_lat_rad10() {
        load_maplibre_gl().await;
        let bounds_exp = Array::from(&JsValue::from(vec![
            -73.975_018_621_413_28,
            40.773_510_168_472_29,
            -73.974_781_378_586_73,
            40.773_689_831_527_71,
        ]));
        let lng_lat_bounds_exp = LngLatBounds::from_corner_values(&bounds_exp);

        let lng_lat = LngLat::new(-73.9749, 40.7736).unwrap();
        let lng_lat_bounds = LngLatBounds::from_lng_lat(&lng_lat, 100.);

        console_log!("{:?}", lng_lat_bounds.to_string());

        assert!((lng_lat_bounds_exp.get_west() - lng_lat_bounds.get_west()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_south() - lng_lat_bounds.get_south()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_east() - lng_lat_bounds.get_east()).abs() < 0.01);
        assert!((lng_lat_bounds_exp.get_north() - lng_lat_bounds.get_north()).abs() < 0.01);
    }
}
