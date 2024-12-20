use crate::api::SubmitSupportRequestAction;
use leptos::*;
use leptos_cosmo::prelude::*;
use leptos_meta as meta;

#[component]
fn SupportForm(
    #[prop(into)] submit_label: MaybeSignal<String>,
    #[prop(into)] message: MaybeSignal<String>,
    #[prop(into)] header: MaybeSignal<String>,
) -> impl IntoView {
    let submit_request_action = create_server_action::<SubmitSupportRequestAction>();

    view! {
        <meta::Title text={
            let header = header.clone();
            move || header.get()
        } />
        <Title title=header />
        <div class="pandas-support">
            {move || {
                if submit_request_action.value().get().is_some_and(|res| res.is_ok()) {
                    Some(
                        view! {
                            <AlertMessage message_type=MessageType::Positive>
                                <MessageContent slot>
                                    {"Deine Nachricht wurde erfolgreich an unser Team geschickt"}
                                </MessageContent>
                            </AlertMessage>
                        },
                    )
                } else {
                    None
                }
            }} <AlertMessage message_type=MessageType::Information>
                <MessageContent slot>{message.clone()}</MessageContent>
            </AlertMessage>
            <ActionForm
                action=submit_request_action
                buttons=Box::new(move || {
                    view! { <Button label=submit_label is_submit=true variant=Variant::Primary /> }
                        .into()
                })
            >
                <Textbox width=InputWidth::Large required=true name="subject" label="Betreff" />
                <Textarea
                    width=InputWidth::Large
                    rows=20
                    required=true
                    name="message"
                    label="Nachricht"
                />
            </ActionForm>
        </div>
    }
}

#[component]
pub fn BambooSupportPage() -> impl IntoView {
    let selected_index = create_rw_signal(0);

    view! {
        <SideList selected_index>
            <ListItem label="Ich habe einen Fehler gefunden" slot>
                <SupportForm
                    header="Melde uns einen Fehler"
                    submit_label="Fehler melden"
                    message="Du hast einen Fehler gefunden? Kein Problem, schreib bitte genau auf wie wir diesen Fehler nachstellen können und wir kümmern uns um einen Fix. Du bekommst eine Email mit Infos zum Status"
                />
            </ListItem>
            <ListItem label="Ich habe eine Frage" slot>
                <SupportForm
                    header="Frag uns was"
                    submit_label="Frage stellen"
                    message="Du hast eine Frage an uns? Kein Problem, schreib einfach was du von uns wissen willst, wir werden unser Bestes geben deine Frage zu beantworten. Die Antwort bekommst du an die Emailadresse die in deinem Account eingerichtet ist"
                />
            </ListItem>
            <ListItem label="Hallo 👋" slot>
                <SupportForm
                    header="Hallo auch an dich 🐼"
                    submit_label="Nachricht senden"
                    message="Du willst einfach mit uns reden und Hallo sagen? Dann schreib uns einfach deine Nachricht wir freuen uns immer von den Pandas im Bambushain zu hören"
                />
            </ListItem>
        </SideList>
    }
}
