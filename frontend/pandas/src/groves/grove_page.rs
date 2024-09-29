use crate::api::is_grove_mod;
use crate::components;
use crate::groves::grove_admin::GroveAdminTab;
use leptos::*;
use leptos_cosmo::prelude::*;
use leptos_router::use_params_map;
use std::str::FromStr;

#[component]
pub fn GrovePage() -> impl IntoView {
    let selected_index = create_rw_signal(0usize);

    let params = use_params_map();
    let id = create_memo(move |_| {
        i32::from_str(
            params
                .get()
                .get("id")
                .cloned()
                .unwrap_or("-1".to_string())
                .as_str(),
        )
        .unwrap_or(-1)
    });
    let name = create_memo(move |_| params.get().get("name").cloned().unwrap_or_default());

    let is_grove_mod_resource = create_local_resource(
        move || id.get(),
        move |id| async move { is_grove_mod(id).await },
    );

    let is_mod = RwSignal::new(false);

    create_effect(move |_| {
        if let Some(Ok(is_grove_mod)) = is_grove_mod_resource.get() {
            is_mod.set(is_grove_mod);
        }
    });

    view! {
        <leptos_meta::Title text=move || name.get() />
        <span class="cosmo-title">{name}</span>
        <Show
            when=move || is_mod.get()
            fallback=move || {
                view! {
                    <TabControl selected_index>
                        <TabItem label="Event Kalender" slot>
                            <div class="pandas-grove__content">
                                <components::Calendar grove_id=id.get() />
                            </div>
                        </TabItem>
                        <TabItem label="Pandas" slot>
                            <div class="pandas-grove__content">
                                <components::PandasList grove_id=id.get() />
                            </div>
                        </TabItem>
                        <TabItem label="Modbereich" slot>
                            <GroveAdminTab grove_id=id.get() grove_name=name.get() />
                        </TabItem>
                    </TabControl>
                }
            }
        >
            <TabControl selected_index>
                <TabItem label="Event Kalender" slot>
                    <div class="pandas-grove__content">
                        <components::Calendar grove_id=id.get() />
                    </div>
                </TabItem>
                <TabItem label="Pandas" slot>
                    <div class="pandas-grove__content">
                        <components::PandasList grove_id=id.get() />
                    </div>
                </TabItem>
                <TabItem label="Modbereich" slot>
                    <GroveAdminTab grove_id=id.get() grove_name=name.get() />
                </TabItem>
            </TabControl>
        </Show>
    }
}
