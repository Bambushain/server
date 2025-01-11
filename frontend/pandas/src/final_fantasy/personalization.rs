use crate::api::ff::{
    get_free_companies, CreateFreeCompanyAction, DeleteFreeCompanyAction, EditFreeCompanyAction,
};
use crate::components::{Card, CardBottom, CardList};
use bamboo_common::core::entities::FreeCompanyWithCharacterCount;
use leptos::prelude::*;
use leptos_cosmo::prelude::*;

#[component]
pub fn CustomFields() -> impl IntoView {
    view! {}
}

#[component]
fn CreateFreeCompanyDialog(
    #[prop(into)] on_save: Callback<(), ()>,
    #[prop(into)] on_close: Callback<(), ()>,
) -> impl IntoView {
    let action = ServerAction::<CreateFreeCompanyAction>::new();
    let value = action.value();

    Effect::new(move |_| {
        if value.read().is_some() {
            on_save.run(())
        }
    });

    view! {
        <ActionFormModal action=action title="Freie Gesellschaft hinzufügen">
            <ModalContent slot>
                <Textbox required=true label="Name" name="name" />
            </ModalContent>
            <ModalButton on_click=on_close label="Schließen" slot />
            <ModalButton is_submit=true label="Freie Gesellschaft hinzufügen" slot />
        </ActionFormModal>
    }
}

#[component]
fn EditFreeCompanyDialog(
    #[prop(into)] name: Signal<String>,
    #[prop(into)] id: Signal<i32>,
    #[prop(into)] on_save: Callback<(), ()>,
    #[prop(into)] on_close: Callback<(), ()>,
) -> impl IntoView {
    let action = ServerAction::<EditFreeCompanyAction>::new();
    let value = action.value();

    let name = RwSignal::new(name.get());

    Effect::new(move |_| {
        if value.read().is_some() {
            on_save.run(())
        }
    });

    view! {
        <ActionFormModal action=action title=format!("{} bearbeiten", name.read().to_string())>
            <ModalContent slot>
                <input type="hidden" value=id name="id" />
                <Textbox required=true label="Name" name="name" value=name />
            </ModalContent>
            <ModalButton on_click=on_close label="Änderungen verwerfen" slot />
            <ModalButton is_submit=true label="Freie Gesellschaft speichern" slot />
        </ActionFormModal>
    }
}

#[component]
pub fn FreeCompanies() -> impl IntoView {
    let free_companies_resource =
        Resource::new(|| (), |_| async move { get_free_companies().await });

    let delete_free_company_action = ServerAction::<DeleteFreeCompanyAction>::new();

    let add_free_company = RwSignal::new(false);

    let selected_free_company_name = RwSignal::new(String::default());
    let selected_free_company_id = RwSignal::new(-1);
    let edit_open = RwSignal::new(false);

    let delete_free_company = move |id, name| {
        use_modals().confirm(
            format!("{name} löschen?"),
            format!("Soll die freie Gesellschaft {name} wirklich gelöscht werden?"),
            Variant::Negative,
            format!("{name} löschen"),
            format!("{name} behalten"),
            Some(Callback::new(move |_| {
                delete_free_company_action.dispatch(DeleteFreeCompanyAction { id });
            })),
            None,
        )
    };
    let edit_free_company = move |free_company: FreeCompanyWithCharacterCount| {
        selected_free_company_name.set(free_company.name.clone());
        selected_free_company_id.set(free_company.id);
        edit_open.set(true);
    };

    let edit_saved = Callback::from(move || {
        free_companies_resource.refetch();
        edit_open.set(false)
    });

    let add_saved = Callback::from(move || {
        free_companies_resource.refetch();
        add_free_company.set(false)
    });

    Effect::new(move |_| {
        if delete_free_company_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            free_companies_resource.refetch();
        }
    });

    view! {
        <leptos_meta::Title text="Freie Gesellschaften" />
        <Transition fallback=|| view! { <ProgressRing /> }>
            <div class="pandas-free-companies">
                <CardList>
                    {move || {
                        Suspend::new(async move {
                            free_companies_resource
                                .await
                                .map(move |free_companies| {
                                    free_companies
                                        .into_iter()
                                        .map(move |item| {
                                            let name = item.name.clone();
                                            let id = item.id;
                                            let item = item.clone();
                                            let free_company_to_edit = item.clone();

                                            view! {
                                                <Card title=name.clone()>
                                                    <KeyValueList>
                                                        <dt>Charaktere</dt>
                                                        <dd>{item.character_count}</dd>
                                                    </KeyValueList>
                                                    <CardBottom slot>
                                                        <Button
                                                            label="Bearbeiten"
                                                            on:click=move |_| edit_free_company(
                                                                free_company_to_edit.clone(),
                                                            )
                                                        />
                                                        <Button
                                                            label="Löschen"
                                                            on:click=move |_| delete_free_company(id, name.clone())
                                                        />
                                                    </CardBottom>
                                                </Card>
                                            }
                                        })
                                        .collect_view()
                                })
                        })
                    }}
                </CardList>
                <CircleButton
                    size=CircleButtonSize::Large
                    variant=Variant::Primary
                    icon=icons::LuPlus
                    title="Freie Gesellschaft hinzufügen"
                    on:click=move |_| add_free_company.set(true)
                />
                <Show when=move || edit_open.get()>
                    <EditFreeCompanyDialog
                        name=selected_free_company_name
                        id=selected_free_company_id
                        on_save=edit_saved
                        on_close=move || edit_open.set(false)
                    />
                </Show>
                <Show when=move || add_free_company.get()>
                    <CreateFreeCompanyDialog
                        on_save=add_saved
                        on_close=move || add_free_company.set(false)
                    />
                </Show>
            </div>
        </Transition>
    }
}
