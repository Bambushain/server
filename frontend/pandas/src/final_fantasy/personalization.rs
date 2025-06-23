use crate::api::ff::{
    create_custom_field, get_custom_fields, get_free_companies, save_custom_field_position,
    update_custom_field, CreateFreeCompanyAction, DeleteCustomFieldAction, DeleteFreeCompanyAction,
    EditFreeCompanyAction,
};
use crate::api::BambooCodeError;
use crate::components::{Card, CardBottom, CardList};
use bamboo_common::core::entities::{
    CustomCharacterField, CustomCharacterFieldOption, FreeCompanyWithCharacterCount,
};
use leptos::ev::{DragEvent, SubmitEvent};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_cosmo::icons::*;
use leptos_cosmo::prelude::*;
use std::collections::BTreeSet;

#[component]
fn EditCustomFieldDialog(
    #[prop(into)] custom_field: CustomCharacterField,
    on_close: Callback<(), ()>,
    on_save: Callback<(), ()>,
) -> impl IntoView {
    let label = RwSignal::new(custom_field.label.clone());
    let new_option = RwSignal::new("".to_string());
    let error_message = RwSignal::new("".to_string());
    let error_message_header = RwSignal::new("".to_string());
    let values = RwSignal::new(
        custom_field
            .clone()
            .options
            .into_iter()
            .map(|option| option.label)
            .collect::<BTreeSet<_>>(),
    );
    let deleted_values = RwSignal::new(vec![]);

    let has_error = RwSignal::new(false);

    let update_custom_field = {
        let custom_field = custom_field.clone();

        move |ev: SubmitEvent| {
            ev.prevent_default();
            let id = custom_field.id;
            let position = custom_field.position as usize;
            let label = label.get();
            let values = values.get();
            let deleted_values = deleted_values.get().into_iter().collect::<BTreeSet<_>>();
            let values = values
                .into_iter()
                .map(|value| {
                    if let Some(option) = custom_field
                        .options
                        .iter()
                        .find(|option| option.label == value)
                    {
                        (option.id, value)
                    } else {
                        (-1, value)
                    }
                })
                .collect::<BTreeSet<_>>();

            spawn_local(async move {
                has_error.set(match update_custom_field(
                    id,
                    position,
                    label,
                    Some(values),
                    Some(deleted_values),
                )
                    .await
                {
                    Err(BambooCodeError::ExistsAlready) =>
                        {
                            error_message_header.set("Feld existiert".to_string());
                            error_message.set("Du hast schon ein Feld mit diesem Namen".to_string());
                            true
                        }
                    Err(_) => {
                        error_message_header.set("Fehler beim Speichern".to_string());
                        error_message.set("Ein unbekannter Fehler ist aufgetreten, bitte wende dich an den Bambussupport".to_string());
                        true
                    }
                    Ok(_) => {
                        on_save.run(());
                        false
                    }
                });
            })
        }
    };

    Effect::new(move |_| {});

    let delete_option = {
        let custom_field = custom_field.clone();

        Callback::new(move |field| {
            if let Some(CustomCharacterFieldOption { id, .. }) = custom_field
                .clone()
                .options
                .iter()
                .find(|option| option.label == field)
            {
                deleted_values.update(|values| values.push(*id));
            }
            values.update(|values| values.retain(|label| label != &field));
        })
    };
    let add_field = move |ev: SubmitEvent| {
        ev.prevent_default();
        if !new_option.read().is_empty() {
            values.update(|values| {
                values.insert(new_option.get());
            });
            new_option.set("".to_string());
        }
    };

    view! {
        <form id="new_option" on:submit=add_field />
        <FormModal
            title="Feld bearbeiten"
            has_error=has_error
            error_message=error_message
            error_message_header=error_message_header
            on:submit=update_custom_field
        >
            <ModalContent slot>
                <Textbox label="Feldname" value=label />
                <label class="cosmo-label" for="new-option">
                    Neues Feld
                </label>
                <div class="cosmo-input is--group">
                    <input
                        class="cosmo-input"
                        id="new-option"
                        type="text"
                        bind:value=new_option
                        form="new_option"
                    />
                    <button
                        type="submit"
                        class="cosmo-button is--primary is--addon"
                        form="new_option"
                    >
                        <Icon width="1rem" height="1rem" icon=LuPlus />
                    </button>
                </div>
                {move || {
                    values
                        .get()
                        .into_iter()
                        .map(|label| {
                            view! {
                                <div class="pandas-custom-fields__option is--new">
                                    <span>{label.clone()}</span>
                                    <button
                                        type="button"
                                        class="cosmo-button is--negative is--custom-fields is--edit"
                                        on:click=move |_| delete_option.run(label.clone())
                                    >
                                        <Icon width="1.25rem" height="1.25rem" icon=LuTrash />
                                    </button>
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </ModalContent>
            <ModalButton on_click=on_close label="Änderungen verwerfen" slot />
            <ModalButton is_submit=true label="Feld speichern" slot />
        </FormModal>
    }
}

#[component]
fn CreateCustomFieldDialog(
    #[prop(into)] position: Signal<i32>,
    on_close: Callback<(), ()>,
    on_save: Callback<(), ()>,
) -> impl IntoView {
    let label = RwSignal::new("".to_string());
    let new_option = RwSignal::new("".to_string());
    let error_message = RwSignal::new("".to_string());
    let error_message_header = RwSignal::new("".to_string());
    let values = RwSignal::new(BTreeSet::<String>::new());

    let has_error = RwSignal::new(false);

    let create_custom_field = move |ev: SubmitEvent| {
        ev.prevent_default();
        let position = position.get() as usize;
        let label = label.get();
        let values = values.get().into_iter().collect::<BTreeSet<_>>();

        spawn_local(async move {
            has_error.set(match create_custom_field(position, label, values).await {
                Err(BambooCodeError::ExistsAlready) =>
                {
                    error_message_header.set("Feld existiert".to_string());
                    error_message.set("Du hast schon ein Feld mit diesem Namen".to_string());
                    true
                }
                Err(_) => {
                    error_message_header.set("Fehler beim Erstellen".to_string());
                    error_message.set( "Ein unbekannter Fehler ist aufgetreten, bitte wende dich an den Bambussupport".to_string());
                    true
                }
                Ok(_) => {
                    on_save.run(());
                    false
                }
            });
        })
    };

    let delete_option = move |field| {
        values.update(|values| values.retain(|value| value != &field));
    };
    let add_field = move |ev: SubmitEvent| {
        ev.prevent_default();
        if !new_option.read().is_empty() {
            values.update(|values| {
                values.insert(new_option.get());
            });
            new_option.set("".to_string());
        }
    };

    view! {
        <form id="new_option" on:submit=add_field />
        <FormModal
            title="Feld hinzufügen"
            has_error=has_error
            error_message=error_message
            error_message_header=error_message_header
            on:submit=create_custom_field
        >
            <ModalContent slot>
                <Textbox label="Feldname" value=label />
                <label class="cosmo-label" for="new-option">
                    Neues Feld
                </label>
                <div class="cosmo-input is--group">
                    <input
                        class="cosmo-input"
                        id="new-option"
                        type="text"
                        bind:value=new_option
                        form="new_option"
                    />
                    <button
                        type="submit"
                        class="cosmo-button is--primary is--addon"
                        form="new_option"
                    >
                        <Icon width="1rem" height="1rem" icon=LuPlus />
                    </button>
                </div>
                {move || {
                    values
                        .get()
                        .into_iter()
                        .map(|field| {
                            view! {
                                <div class="pandas-custom-fields__option is--new">
                                    <span>{field.clone()}</span>
                                    <button
                                        type="button"
                                        class="cosmo-button is--negative is--custom-fields is--edit"
                                        on:click=move |_| delete_option(field.clone())
                                    >
                                        <Icon width="1.25rem" height="1.25rem" icon=LuTrash />
                                    </button>
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </ModalContent>
            <ModalButton on_click=on_close label="Schließen" slot />
            <ModalButton is_submit=true label="Feld hinzufügen" slot />
        </FormModal>
    }
}

#[component]
pub fn CustomFields() -> impl IntoView {
    let custom_fields_resource = Resource::new(|| (), |_| async move { get_custom_fields().await });

    let delete_custom_field_action = ServerAction::<DeleteCustomFieldAction>::new();

    let custom_fields = RwSignal::new(vec![] as Vec<CustomCharacterField>);

    let add_open = RwSignal::new(false);
    let add_position = RwSignal::new(-1);

    let edit_open = RwSignal::new(false);
    let edit_field = RwSignal::new(CustomCharacterField::default());

    let dragging_item = RwSignal::new(None as Option<CustomCharacterField>);
    let dragging_item_idx = RwSignal::new(0usize);

    let dragstart = move |idx: usize, field: CustomCharacterField| {
        dragging_item.set(Some(field));
        dragging_item_idx.set(idx);
    };
    let dragover = move |ev: DragEvent, idx: usize| {
        ev.prevent_default();
        if dragging_item_idx.read() == idx {
            return;
        }

        custom_fields.update(move |custom_fields| custom_fields.swap(dragging_item_idx.get(), idx));
        dragging_item_idx.set(idx);
    };
    let save_changes = move |_| {
        if let Some(id) = dragging_item.get().map(|item| item.id) {
            let position = dragging_item_idx.get();
            spawn_local(async move {
                let _ = save_custom_field_position(id, position as i32).await;
                if let Ok(fields) = get_custom_fields().await {
                    custom_fields.set(fields);
                }
            })
        }
    };

    let delete_field = move |idx| {
        let field: Option<CustomCharacterField> = custom_fields.get().get(idx).cloned();

        if let Some(field) = field {
            use_modals().confirm(
                "Feld löschen?",
                format!(
                    "Soll das Feld {} wirklich gelöscht werden?",
                    field.label.clone()
                ),
                Variant::Negative,
                "Feld löschen",
                "Nicht löschen",
                Some(Callback::new(move |_| {
                    delete_custom_field_action.dispatch(DeleteCustomFieldAction { id: field.id });
                })),
                None,
            )
        }
    };

    let add_saved = Callback::new(move |_| {
        add_open.set(false);
        custom_fields_resource.refetch()
    });
    let open_add = move |position| {
        add_open.set(true);
        add_position.set(position);
    };

    let edit_saved = Callback::new(move |_| {
        edit_open.set(false);
        custom_fields_resource.refetch()
    });
    let open_edit = move |field| {
        edit_open.set(true);
        edit_field.set(field);
    };

    Effect::new(move |_| {
        if delete_custom_field_action
            .value()
            .get()
            .is_some_and(|res| res.is_ok())
        {
            custom_fields_resource.refetch()
        }
    });

    view! {
        <Transition fallback=|| {
            view! { <ProgressRing /> }
        }>
            {move || {
                Suspend::new(async move {
                    custom_fields_resource
                        .await
                        .map(|fields| {
                            custom_fields.set(fields);
                        })
                })
            }} <leptos_meta::Title text="Eigene Felder" />
            <div class="pandas-custom-fields__container">
                {move || {
                    custom_fields
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(idx, field)| {
                            let position = field.position;
                            let edit_field = field.clone();

                            view! {
                                <button
                                    class="cosmo-button is--circle is--primary is--add is--custom-fields"
                                    on:click=move |_| open_add(position)
                                >
                                    <Icon icon=LuPlus width="1.25rem" height="1.25rem" />
                                </button>
                                <div
                                    class="pandas-custom-fields__item"
                                    on:dragstart=move |_| dragstart(idx, field.clone())
                                    on:dragover=move |ev| dragover(ev, idx)
                                    on:dragend=save_changes
                                    draggable="true"
                                >
                                    <span>{field.label.clone()}</span>
                                    <div class="pandas-custom-fields__buttons is--edit">
                                        <button
                                            class="cosmo-button is--primary is--custom-fields is--edit"
                                            on:click=move |_| open_edit(edit_field.clone())
                                        >
                                            <Icon width="1.25rem" height="1.25rem" icon=LuPencil />
                                        </button>
                                        <button
                                            class="cosmo-button is--negative is--custom-fields is--edit"
                                            on:click=move |_| delete_field(idx)
                                        >
                                            <Icon width="1.25rem" height="1.25rem" icon=LuTrash />
                                        </button>
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()
                }}
                <button
                    class="cosmo-button is--circle is--primary is--add is--custom-fields"
                    on:click=move |_| open_add(custom_fields.get().len() as i32)
                >
                    <Icon icon=LuPlus width="1.25rem" height="1.25rem" />
                </button> <Show when=move || add_open.get()>
                    <CreateCustomFieldDialog
                        position=add_position
                        on_close=Callback::new(move |_| add_open.set(false))
                        on_save=add_saved
                    />
                </Show> <Show when=move || edit_open.get()>
                    <EditCustomFieldDialog
                        custom_field=edit_field.get()
                        on_close=Callback::new(move |_| edit_open.set(false))
                        on_save=edit_saved
                    />
                </Show>
            </div>
        </Transition>
    }
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
