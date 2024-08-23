use leptos::*;
use leptos_meta as meta;
use crate::components;

#[component]
pub fn Calendar() -> impl IntoView {
    view! {
        <meta::Title text="Event Kalender" />
        <components::Calendar />
    }
}
