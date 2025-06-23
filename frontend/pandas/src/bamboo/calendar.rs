use crate::components;
use leptos::prelude::*;
use leptos_meta as meta;

#[component]
pub fn Calendar() -> impl IntoView {
    view! {
        <meta::Title text="Event Kalender" />
        <components::Calendar />
    }
}
