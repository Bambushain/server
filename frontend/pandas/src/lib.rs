pub mod api;
pub mod app;
#[cfg(feature = "ssr")]
pub mod authentication;
mod bamboo;
mod components;
mod final_fantasy;
mod groves;
mod my;
mod state;
mod support;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::prelude::*;

    console_error_panic_hook::set_once();

    hydrate_body(App);
}
