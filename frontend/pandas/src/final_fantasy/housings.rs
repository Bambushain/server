use crate::api::ff::{get_housings, DeleteHousingAction};
use crate::components::*;
use leptos::*;
use leptos_cosmo::prelude::*;

#[component]
pub fn HousingTab(character_id: MaybeSignal<i32>) -> impl IntoView {
    let housing_resource = create_resource(
        move || character_id.get(),
        |id| async move { get_housings(id).await },
    );
    let delete_housing_action = create_server_action::<DeleteHousingAction>();

    let delete_housing = {
        let housing_resource = housing_resource.clone();
        let delete_housing_action = delete_housing_action.clone();

        move |housing_id: i32| {
            if let Some(Some(Some(housing))) = housing_resource.get().map(|res| {
                res.ok()
                    .map(|res| res.iter().cloned().find(|f| f.id == housing_id).clone())
            }) {
                confirm(
                    "Unterkunft löschen",
                    format!(
                        "Soll die {} im Gebiet {} im Bezirk {} mit der Nummer {} wirklich gelöscht werden?",
                        housing.housing_type,
                        housing.district,
                        housing.ward,
                        housing.plot
                    ),
                    Variant::Negative,
                    "Unterkunft löschen",
                    "Unterkunft behalten",
                    Some(Callback::new(move |_| {
                        delete_housing_action.dispatch(DeleteHousingAction {
                            housing_id,
                            character_id: character_id.get(),
                        })
                    })),
                    None,
                );
            }
        }
    };

    create_effect(move |_| {
        if delete_housing_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            housing_resource.refetch()
        }
    });

    view! {
        <Transition fallback=|| view! {<ProgressRing />}>
            <div class="pandas-character-tab is--housing">
                <Show when=move || housing_resource.get().is_some_and(|res| res.is_ok_and(|res| !res.is_empty())) fallback=|| view! {
                    <AlertMessage header="Noch keine Unterkünfte" message_type=MessageType::Information>
                        <MessageContent slot>
                            "Du hast noch keine Unterkünfte angelegt, klick unten auf das Plus um deine Erste anzulegen"
                        </MessageContent>
                    </AlertMessage>
                }>
                    <CardList>
                        {move || {
                            housing_resource
                                .get()
                                .map(|housings| {
                                    housings
                                        .ok()
                                        .map(|housings| {
                                            housings
                                                .iter()
                                                .cloned()
                                                .map(|housing| {
                                                    view! {
                                                        <Card title=housing.district.to_string()>
                                                            {housing.housing_type.to_string()}<br />
                                                            {format!("Bezirk {}", housing.ward)}<br />
                                                            {format!("Nr. {}", housing.plot)}
                                                            <CardBottom slot>
                                                                <Button label="Bearbeiten" />
                                                                <Button label="Löschen" on:click=move |_| delete_housing(housing.id) />
                                                            </CardBottom>
                                                        </Card>
                                                    }
                                                })
                                                .collect_view()
                                        })
                                })
                        }}
                    </CardList>
                </Show>
                <CircleButton size=CircleButtonSize::Large variant=Variant::Primary icon=icons::LuPlus title="Kämpfer erstellen" />
            </div>
        </Transition>
    }
}
