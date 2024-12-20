use crate::api::ff::{get_housings, CreateHousingAction, DeleteHousingAction};
use crate::components::*;
use bamboo_common::core::entities::{CharacterHousing, HousingDistrict, HousingType};
use leptos::*;
use leptos_cosmo::prelude::*;
use strum::IntoEnumIterator;

#[component]
fn CreateHousingDialog(
    character_id: MaybeSignal<i32>,
    housings: MaybeSignal<Option<Option<Vec<CharacterHousing>>>>,
    on_save: Callback<(), ()>,
    on_close: Callback<(), ()>,
) -> impl IntoView {
    let action = create_server_action::<CreateHousingAction>();
    let selected_type = create_rw_signal(Some(HousingType::Private.get_name()));
    let selected_plot = create_rw_signal(Some("1".to_string()));
    let selected_ward = create_rw_signal(Some("1".to_string()));

    let types = create_memo(|_| {
        HousingType::iter()
            .map(|t| (Some(t.get_name()), t.to_string()))
            .collect::<Vec<_>>()
    });
    let districts = {
        let housings = housings.clone();

        create_memo(move |_| {
            let housings = housings.get().unwrap().unwrap();

            HousingDistrict::iter()
                .filter(|district| {
                    housings
                        .iter()
                        .filter(|housing| housing.district == *district)
                        .count()
                        < 30 * 60
                })
                .map(|district| (Some(district.get_name()), district.to_string()))
                .collect::<Vec<_>>()
        })
    };
    let selected_district = create_rw_signal(Some(HousingDistrict::TheLavenderBeds.get_name()));
    let wards = {
        let selected_district = selected_district.clone();
        let housings = housings.clone();

        create_memo(move |_| {
            let selected_district = selected_district.get().unwrap();
            let plots = housings
                .get()
                .unwrap()
                .unwrap()
                .iter()
                .cloned()
                .filter(|housing| housing.district.get_name() == selected_district)
                .collect::<Vec<_>>();
            (1..=30i16)
                .filter(|ward| plots.iter().filter(|plot| plot.ward == *ward).count() < 60)
                .map(|ward| (Some(ward.to_string()), ward.to_string()))
                .collect::<Vec<_>>()
        })
    };
    let plots = create_memo(move |_| {
        let selected_district = selected_district.get().unwrap();
        let selected_ward = selected_ward.get().unwrap();
        let plots = housings
            .get()
            .unwrap()
            .unwrap()
            .iter()
            .filter(|housing| {
                housing.district.get_name() == selected_district
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

    create_effect(move |_| {
        if value.get().is_some() {
            on_save.call(())
        }
    });
    create_effect(move |_| selected_district.set(districts.get().first().unwrap().0.clone()));
    create_effect(move |_| selected_ward.set(wards.get().first().unwrap().0.clone()));
    create_effect(move |_| selected_plot.set(plots.get().first().unwrap().0.clone()));

    view! {
        <ActionFormModal action=action title="Unterkunft hinzufügen" >
            <ModalContent slot>
                <input type="hidden" value=character_id name="character_id" />
                <SingleSelect label="Typ" items=types selected=selected_type name="housing_type" />
                <SingleSelect label="Gebiet" items=districts selected=selected_district name="district" />
                <SingleSelect label="Bezirk" items=wards selected=selected_ward name="ward" />
                <SingleSelect label="Nummer" items=plots selected=selected_plot name="plot" />
            </ModalContent>
            <ModalButton on_click=on_close label="Schließen" slot />
            <ModalButton is_submit=true label="Unterkunft hinzufügen" slot />
        </ActionFormModal>
    }
}

#[component]
pub fn HousingTab(character_id: MaybeSignal<i32>) -> impl IntoView {
    let housing_resource = create_resource(
        move || character_id.get(),
        |id| async move { get_housings(id).await },
    );
    let add_open = create_rw_signal(false);
    let add_saved = Callback::from(move |_| {
        housing_resource.refetch();
        add_open.set(false)
    });

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
                <CircleButton size=CircleButtonSize::Large variant=Variant::Primary icon=icons::LuPlus title="Unterkunft erstellen" on:click=move |_| add_open.set(true) />
                <Show when=move || add_open.get()>
                    <CreateHousingDialog housings=housing_resource.get().map(|housings| housings.ok()).into() character_id=character_id on_close=Callback::from(move |_| add_open.set(false)) on_save=add_saved />
                </Show>
            </div>
        </Transition>
    }
}
