use crate::api::{is_banned_from_grove, is_grove_mod};
use crate::components;
use crate::groves::grove_admin::GroveAdminTab;
use leptos::prelude::*;
use leptos_cosmo::prelude::*;
use leptos_router::hooks::use_params_map;
use std::str::FromStr;

#[component]
pub fn GrovePage() -> impl IntoView {
    let selected_index = RwSignal::new(0usize);

    let params = use_params_map();
    let id = Memo::new(move |_| {
        Some(
            i32::from_str(params.read().get("id").unwrap_or("-1".to_string()).as_str())
                .unwrap_or(-1),
        )
    });
    let name = Memo::new(move |_| params.read().get("name").unwrap_or_default());

    let is_grove_mod_resource = Resource::new(
        move || id.get(),
        move |id| async move { is_grove_mod(id.unwrap()).await },
    );
    let is_banned_from_grove_resource = Resource::new(
        move || id.get(),
        move |id| async move { is_banned_from_grove(id.unwrap()).await },
    );

    view! {
        <Transition>
            {move || {
                Suspend::new(async move {
                    let is_banned = is_banned_from_grove_resource.await.is_ok_and(|res| res);
                    let is_mod = is_grove_mod_resource.await.is_ok_and(|res| res);
                    if !is_mod && selected_index.read() == 2 {
                        selected_index.set(0);
                    }

                    view! {
                        <Show when=move || !is_banned fallback=move || view! {
                            <leptos_meta::Title text="Gebannt" />
                            <AlertMessage header="Du bist gebannt" message_type=MessageType::Negative>
                                <MessageContent slot>
                                    Tut uns leid, aber du wurdest aus diesem Hain gebannt. Bitte wähle einen anderen.
                                </MessageContent>
                            </AlertMessage>
                        }>
                            <leptos_meta::Title text=move || name.get() />
                            <span class="cosmo-title">{name}</span>
                            <Show
                                when=move || is_mod
                                fallback=move || {
                                    view! {
                                        <TabControl selected_index>
                                            <TabItem label="Event Kalender" slot>
                                                <div class="pandas-grove__content">
                                                    <components::Calendar grove_id=id />
                                                </div>
                                            </TabItem>
                                            <TabItem label="Pandas" slot>
                                                <div class="pandas-grove__content">
                                                    <components::PandasList grove_id=id />
                                                </div>
                                            </TabItem>
                                        </TabControl>
                                    }
                                }
                            >
                                <TabControl selected_index>
                                    <TabItem label="Event Kalender" slot>
                                        <div class="pandas-grove__content">
                                            <components::Calendar grove_id=id />
                                        </div>
                                    </TabItem>
                                    <TabItem label="Pandas" slot>
                                        <div class="pandas-grove__content">
                                            <components::PandasList grove_id=id />
                                        </div>
                                    </TabItem>
                                    <TabItem label="Modbereich" slot>
                                        <GroveAdminTab grove_id=id grove_name=name />
                                    </TabItem>
                                </TabControl>
                            </Show>
                        </Show>
                    }
                })
            }}
        </Transition>
    }
}
