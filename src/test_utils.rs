//! # Shared test functions

use std::marker::PhantomData;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    Document, HtmlElement, HtmlScriptElement,
    js_sys::{self, Reflect},
};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

const MAPLIBRE_SCRIPT_SRC: &str = "https://unpkg.com/maplibre-gl@^5.12.0/dist/maplibre-gl.js";
const MAPLIBRE_ID: &str = "maplibre-gl-e1b6fcf0-2a8c-46ae-9c7e-deeb271d32d7";

/// `web_sys::HtmlElement` which will clean up the element when it is dropped
pub struct HtmlElementRAII<'a> {
    html_element: HtmlElement,
    lifetime: PhantomData<&'a ()>,
}

impl Drop for HtmlElementRAII<'_> {
    fn drop(&mut self) {
        self.html_element.remove();
    }
}

impl HtmlElementRAII<'_> {
    /// Get a copy of the underlying `web_sys::HtmlElement`
    #[must_use]
    pub fn cloned_ref(&self) -> HtmlElement {
        self.html_element.clone()
    }
}

/// Insert an arbitrary HTML element with a given tag.
///
/// This function assumes that the document and the body are loaded and
/// available
///
/// # Panics
///
/// This function is only intended to run on tests, errors aren't handled and
/// panics whenever anything goes wrong.
#[must_use]
pub fn gen_html_element<'a>(tag: &str) -> HtmlElementRAII<'a> {
    let element = get_document()
        .create_element(tag)
        .expect("Creating an element should work");

    let element_js: JsValue = element.into();
    let html_element = HtmlElement::from(element_js);

    get_document()
        .body()
        .expect("Document should have a body")
        .append_child(&html_element)
        .expect("Appending child should work");

    HtmlElementRAII {
        html_element,
        lifetime: PhantomData,
    }
}

/// Insert an arbitrary HTML element with a given tag and id.
///
/// This function assumes that the document and the body are loaded and
/// available
///
/// # Panics
///
/// This function is only intended to run on tests, errors aren't handled and
/// panics whenever anything goes wrong.
#[must_use]
pub fn gen_html_element_with_id<'a>(tag: &str, id: &str) -> HtmlElementRAII<'a> {
    let element = gen_html_element(tag);
    element.html_element.set_id(id);
    element
}

fn get_document() -> Document {
    let window = web_sys::window().expect("Should be able to get the window");
    window.document().expect("Document should be available")
}

fn load_maplibre_gl_inner(resolve: js_sys::Function, reject: js_sys::Function) {
    let on_error = Closure::wrap(Box::new(move || {
        reject
            .call0(&JsError::new(format!("Error loading {MAPLIBRE_ID}").as_str()).into())
            .expect("Calling reject should work");
    }) as Box<dyn FnMut()>);

    let insert_maplibre_if_not_there = Closure::wrap(Box::new(move || {
        // Check that we don't have maplibre already inserted
        if get_document().get_element_by_id(MAPLIBRE_ID).is_some() {
            resolve
                .call0(&JsValue::NULL)
                .expect("Calling resolve should work");
            return;
        }

        // Create script element
        let element: HtmlScriptElement = Into::<JsValue>::into(
            get_document()
                .create_element("script")
                .expect("Creating a script element should work"),
        )
        .into();
        element.set_id(MAPLIBRE_ID);
        element.set_src(MAPLIBRE_SCRIPT_SRC);
        element.set_async(true);
        element.set_onload(Some(&resolve));
        element.set_onerror(Some(on_error.as_ref().unchecked_ref()));

        // Insert
        get_document()
            .head()
            .expect("Head should be available")
            .append_child(&element)
            .expect("Appending script should work");
    }) as Box<dyn FnMut()>);

    if get_document().ready_state() == "loading" {
        get_document()
            .add_event_listener_with_callback(
                "DOMContentLoaded",
                insert_maplibre_if_not_there.as_ref().unchecked_ref(),
            )
            .expect("Adding listener to DOMContentLoaded should work");
    } else {
        insert_maplibre_if_not_there
            .as_ref()
            .unchecked_ref::<js_sys::Function>()
            .call0(&JsValue::NULL)
            .expect("Calling insert should work");
    }
}

/// Insert MapLibre GL into the document and ensure that they are available.
///
/// # Panics
///
/// This function is only intended to run on tests, errors aren't handled and
/// panics whenever anything goes wrong.
pub async fn load_maplibre_gl() {
    JsFuture::from(js_sys::Promise::new(&mut load_maplibre_gl_inner))
        .await
        .expect("Loading MapLibre gl should work");
}

/// Get the value from a `JsValue` given a key
///
/// # Panics
///
/// This function is only intended to run on tests, errors aren't handled and
/// panics whenever anything goes wrong.
#[must_use]
pub fn get_value_from_object(value: &JsValue, key: &str) -> JsValue {
    let key = serde_wasm_bindgen::to_value(key).expect("Conversion from str to JS should work");
    Reflect::get(value, &key).expect("Key in should be in value")
}
