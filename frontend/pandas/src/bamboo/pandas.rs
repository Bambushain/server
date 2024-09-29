use crate::components;
use leptos::*;
use leptos_meta as meta;

#[component]
pub fn Pandas() -> impl IntoView {
    view! {
        <meta::Title text="Pandas" />
        <components::PandasList />
    }
}
