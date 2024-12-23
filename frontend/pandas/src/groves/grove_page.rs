use crate::api::is_grove_mod;
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
        i32::from_str(params.read().get("id").unwrap_or("-1".to_string()).as_str()).unwrap_or(-1)
    });
    let name = Memo::new(move |_| params.read().get("name").unwrap_or_default());

    let is_grove_mod_resource = Resource::new(
        move || id.get(),
        move |id| async move { is_grove_mod(id).await },
    );

    let is_mod = RwSignal::new(false);

    Effect::new(move |_| {
        Suspend::new(async move {
            if let Ok(is_grove_mod) = is_grove_mod_resource.await {
                is_mod.set(is_grove_mod);
            }
        })
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
                            <GroveAdminTab grove_id=id grove_name=name />
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
                    <GroveAdminTab grove_id=id grove_name=name />
                </TabItem>
            </TabControl>
        </Show>
    }
}
