pub mod api;
pub mod app;
#[cfg(feature = "ssr")]
pub mod authentication;
mod bamboo;
mod components;
mod groves;
mod my;
mod state;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
