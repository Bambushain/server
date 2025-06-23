use crate::api::SubmitSupportRequestAction;
use leptos::prelude::*;
use leptos_cosmo::prelude::{ActionForm, *};
use leptos_meta as meta;

#[component]
fn SupportForm(
    #[prop(into)] submit_label: Signal<String>,
    #[prop(into)] message: Signal<String>,
    #[prop(into)] header: Signal<String>,
) -> impl IntoView {
    let submit_request_action = ServerAction::<SubmitSupportRequestAction>::new();

    view! {
        <meta::Title text=move || header.get() />
        <Title title=header />
        <div class="pandas-support">
            {move || {
                submit_request_action
                    .value()
                    .get()
                    .is_some_and(|res| res.is_ok())
                    .then_some(
                        view! {
                            <AlertMessage message_type=MessageType::Positive>
                                <MessageContent slot>
                                    {"Deine Nachricht wurde erfolgreich an unser Team geschickt"}
                                </MessageContent>
                            </AlertMessage>
                        },
                    )
            }} <AlertMessage message_type=MessageType::Information>
                <MessageContent slot>{message}</MessageContent>
            </AlertMessage>
            <ActionForm
                action=submit_request_action
                buttons=Box::new(move || {
                    view! { <Button label=submit_label is_submit=true variant=Variant::Primary /> }
                        .into_any()
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
    let selected_index = RwSignal::new(0);

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
