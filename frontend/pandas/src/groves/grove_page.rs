use crate::api::get_grove;
use crate::components;
use leptos::*;
use leptos_cosmo::prelude::*;
use leptos_router::use_params_map;
use std::str::FromStr;

#[component]
pub fn GrovePage() -> impl IntoView {
    let selected_index = create_rw_signal(0usize);

    let params = use_params_map();
    let id = {
        let params = params.clone();

        create_memo(move |_| {
            i32::from_str(
                params
                    .get()
                    .get("id")
                    .cloned()
                    .unwrap_or("-1".to_string())
                    .as_str(),
            )
            .unwrap_or(-1)
        })
    };

    let grove_resource = create_resource(
        move || id.get(),
        move |id| async move { get_grove(id).await },
    );

    view! {
        <Transition fallback=|| view! { <ProgressRing /> }>
            <Show when={
                let grove_resource = grove_resource.clone();

                move || grove_resource.get().unwrap_or(Err(ServerFnError::new("something failed"))).is_ok()
            } fallback=|| view! {
                <AlertMessage header="Fehler beim Laden" message_type=MessageType::Negative>
                    <MessageContent slot>
                        <p>Leider konnte der Hain nicht geladen werden, bitte wende dich an den Bambussupport.</p>
                    </MessageContent>
                </AlertMessage>
            }>
                <leptos_meta::Title text={
                    let grove_resource = grove_resource.clone();

                    move || grove_resource.get().unwrap().unwrap().name
                } />
                <span class="cosmo-title">{move || grove_resource.get().unwrap().unwrap().name}</span>
            </Show>
        </Transition>
        <TabControl selected_index>
            <TabItem label="Event Kalender" slot>
                <div class="pandas-grove__content">
                    <components::Calendar grove_id={id.get()} />
                </div>
            </TabItem>
            <TabItem label="Pandas" slot>
                <div class="pandas-grove__content">
                    <components::PandasList grove_id={id.get()} />
                </div>
            </TabItem>
        </TabControl>
    }
}
