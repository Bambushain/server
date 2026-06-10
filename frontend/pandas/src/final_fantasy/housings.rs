use crate::api::ff::{
    get_free_company_housing, get_housings, CreateHousingAction, DeleteHousingAction,
};
use crate::components::*;
use bamboo_common::core::entities::{CharacterHousing, HousingDistrict, HousingType};
use leptos::prelude::*;
use leptos_cosmo::prelude::*;
use strum::IntoEnumIterator;

#[component]
fn CreateHousingDialog(
    character_id: Signal<i32>,
    housings: Signal<Vec<CharacterHousing>>,
    on_save: Callback<(), ()>,
    on_close: Callback<(), ()>,
) -> impl IntoView {
    let action = ServerAction::<CreateHousingAction>::new();
    let selected_type = RwSignal::new(Some(HousingType::Private.get_serde_name()));
    let selected_district = RwSignal::new(Some(HousingDistrict::TheLavenderBeds.get_serde_name()));
    let selected_plot = RwSignal::new(Some("1".to_string()));
    let selected_ward = RwSignal::new(Some("1".to_string()));

    let types = Memo::new(move |_| {
        let housings = housings.get();
        let private_allowed = housings
            .iter()
            .filter(|h| h.housing_type == HousingType::Private)
            .count()
            == 0;

        HousingType::iter()
            .filter(|t| (t != &HousingType::Private || private_allowed) && t != &HousingType::FreeCompany)
            .map(|t| (Some(t.get_serde_name()), t.to_string()))
            .collect::<Vec<_>>()
    });
    let districts = Memo::new(move |_| {
        let housings = housings.get();

        HousingDistrict::iter()
            .filter(|district| {
                housings
                    .iter()
                    .filter(|housing| housing.district == *district)
                    .count()
                    < 30 * 60
            })
            .map(|district| (Some(district.get_serde_name()), district.to_string()))
            .collect::<Vec<_>>()
    });
    let wards = Memo::new(move |_| {
        let selected_district = selected_district.get().unwrap();
        let plots = housings
            .get()
            .into_iter()
            .filter(|housing| housing.district.get_serde_name() == selected_district)
            .collect::<Vec<_>>();
        (1..=30i16)
            .filter(|ward| plots.iter().filter(|plot| plot.ward == *ward).count() < 60)
            .map(|ward| (Some(ward.to_string()), ward.to_string()))
            .collect::<Vec<_>>()
    });
    let plots = Memo::new(move |_| {
        let selected_district = selected_district.get().unwrap();
        let selected_ward = selected_ward.get().unwrap();
        let plots = housings
            .get()
            .iter()
            .filter(|housing| {
                housing.district.get_serde_name() == selected_district
                    && housing.ward.to_string() == selected_ward
            })
            .map(|housing| housing.plot)
            .collect::<Vec<_>>();
        (1..=60i16)
            .filter(|plot| !plots.contains(plot))
            .map(|plot| (Some(plot.to_string()), plot.to_string()))
            .collect::<Vec<_>>()
    });

    let value = action.value();

    Effect::new(move |_| {
        if value.read().is_some() {
            on_save.run(())
        }
    });
    Effect::new(move |_| selected_type.set(types.get().first().unwrap().0.clone()));
    Effect::new(move |_| selected_district.set(districts.get().first().unwrap().0.clone()));
    Effect::new(move |_| selected_ward.set(wards.get().first().unwrap().0.clone()));
    Effect::new(move |_| selected_plot.set(plots.get().first().unwrap().0.clone()));

    view! {
        <ActionFormModal action=action title="Unterkunft hinzufügen">
            <ModalContent slot>
                <input type="hidden" value=character_id name="character_id" />
                <SingleSelect label="Typ" items=types selected=selected_type name="housing_type" />
                <SingleSelect
                    label="Gebiet"
                    items=districts
                    selected=selected_district
                    name="district"
                />
                <SingleSelect label="Bezirk" items=wards selected=selected_ward name="ward" />
                <SingleSelect label="Nummer" items=plots selected=selected_plot name="plot" />
            </ModalContent>
            <ModalButton on_click=on_close label="Schließen" slot />
            <ModalButton is_submit=true label="Unterkunft hinzufügen" slot />
        </ActionFormModal>
    }
}

#[component]
pub fn HousingTab(character_id: Signal<i32>) -> impl IntoView {
    let housing_resource = Resource::new(
        move || character_id.get(),
        |id| async move { get_housings(id).await },
    );
    let free_company_housing_resource = Resource::new(
        move || character_id.get(),
        |id| async move { get_free_company_housing(id).await },
    );
    let add_open = RwSignal::new(false);
    let add_saved = Callback::from(move || {
        housing_resource.refetch();
        add_open.set(false)
    });

    let delete_housing_action = ServerAction::<DeleteHousingAction>::new();

    let delete_housing = move |housing_id: i32| {
        if let Some(Some(Some(housing))) = housing_resource.get().map(|res| {
            res.ok()
                .map(|res| res.iter().find(|&f| f.id == housing_id).cloned().clone())
        }) {
            use_modals().confirm(
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
                    });
                })),
                None,
            );
        }
    };

    Effect::new(move |_| {
        if delete_housing_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            housing_resource.refetch()
        }
    });

    view! {
        <Transition fallback=|| view! { <ProgressRing /> }>
            <div class="pandas-character-tab is--housing">
                <Show
                    when=move || {
                        housing_resource
                            .get()
                            .is_some_and(|res| res.is_ok_and(|res| !res.is_empty()))
                    }
                    fallback=|| {
                        view! {
                            <AlertMessage
                                header="Noch keine Unterkünfte"
                                message_type=MessageType::Information
                            >
                                <MessageContent slot>
                                    "Du hast noch keine Unterkünfte angelegt, klick unten auf das Plus um deine Erste anzulegen"
                                </MessageContent>
                            </AlertMessage>
                        }
                    }
                >
                    <CardList>
                        {move || {
                            let housing_resource = housing_resource;
                            Suspend::new(async move {
                                let free_company_housing = free_company_housing_resource
                                    .await
                                    .map(|housing| {
                                        housing
                                            .map(|housing| view! {
                                                <Card title=housing
                                                    .district
                                                    .to_string()>
                                                    "Unterkunft der freien Gesellschaft"<br />
                                                    {format!("Bezirk {}", housing.ward)}<br />
                                                    {format!("Nr. {}", housing.plot)}
                                                    <CardBottom slot>
                                                        <Button
                                                            label="Löschen"
                                                            enabled=false
                                                        />
                                                    </CardBottom>
                                                </Card>
                                            })
                                });
                                housing_resource
                                    .await
                                    .map(|housings| {
                                        view! {
                                            {free_company_housing}
                                            <For
                                                each=move || housings.clone()
                                                key=move |housing| housing.clone()
                                                let(housing)
                                            >
                                                <Card title=housing
                                                    .district
                                                    .to_string()>
                                                    {housing.housing_type.to_string()}<br />
                                                    {format!("Bezirk {}", housing.ward)}<br />
                                                    {format!("Nr. {}", housing.plot)}
                                                    <CardBottom slot>
                                                        <Button
                                                            label="Löschen"
                                                            on:click=move |_| delete_housing(housing.id)
                                                        />
                                                    </CardBottom>
                                                </Card>
                                            </For>
                                        }
                                    })
                            })
                        }}
                    </CardList>
                </Show>
                <CircleButton
                    size=CircleButtonSize::Large
                    variant=Variant::Primary
                    icon=icons::LuPlus
                    title="Unterkunft erstellen"
                    on:click=move |_| add_open.set(true)
                />
                <Show when=move || {
                    add_open.get()
                }>
                    {Suspend::new(async move {
                        housing_resource
                            .await
                            .ok()
                            .map(|housings| {
                                view! {
                                    <CreateHousingDialog
                                        housings=housings.into()
                                        character_id=character_id
                                        on_close=Callback::from(move || add_open.set(false))
                                        on_save=add_saved
                                    />
                                }
                            })
                    })}
                </Show>
            </div>
        </Transition>
    }
}
