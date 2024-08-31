use leptos::*;
use leptos_meta as meta;
use crate::components;

#[component]
pub fn Pandas() -> impl IntoView {
    view! {
        <meta::Title text="Pandas" />
        <components::PandasList />
    }
}
