use crate::api::{get_pandas, BanPandaAction, BanResultCode};
use crate::components::{Card, CardBottom, CardList};
use bamboo_common::core::entities::user::GroveUser;
use bamboo_common::core::entities::User;
use leptos::*;
use leptos_cosmo::prelude::*;

#[component]
fn PandaCard(
    panda: GroveUser,
    #[prop(into)] grove_id: Option<i32>,
    #[prop(into)] me_id: i32,
    #[prop(into)] is_mod: Option<bool>,
    #[prop(into)] ban_callback: Callback<BanResultCode>,
) -> impl IntoView {
    let profile_picture = format!(
        "/api/user/{}/picture#time={}",
        panda.id,
        chrono::offset::Local::now().timestamp_millis()
    );
    let display_name = panda.display_name.clone();
    let discord_name = panda.discord_name.clone();

    let ban_panda_action = create_server_action::<BanPandaAction>();

    let ban_panda_confirm = {
        let display_name = display_name.clone();

        let grove_id = grove_id.clone();
        let user_id = panda.id;

        move |_| {
            let grove_id = grove_id.unwrap();

            confirm(
                "Panda bannen",
                format!("Soll der Panda {display_name} wirklich gebannt werden?"),
                Variant::Negative,
                format!("{display_name} bannen"),
                format!("{display_name} nicht bannen"),
                Some(Callback::new(move |_| {
                    ban_panda_action.dispatch(BanPandaAction { grove_id, user_id })
                })),
                None,
            )
        }
    };

    let panda_card_content = view! {
        <a href={format!("mailto:{}", panda.email.clone())}>{panda.email.clone()}</a>
        <Show when={
            let discord_name = discord_name.clone();

            move || !discord_name.is_empty()
        }>
            <span>{"Auf Discord bekannt als "}<strong>{discord_name.clone()}</strong></span>
        </Show>
    };

    {
        let ban_panda_action = ban_panda_action.clone();
        create_effect(move |_| {
            if let Some(Ok(result)) = ban_panda_action.value().get() {
                ban_callback.call(result)
            }
        });
    }

    if grove_id.is_some() && is_mod.unwrap_or(false) {
        let ban_panda_confirm = ban_panda_confirm.clone();

        view! {
            <Card title={display_name.clone()} prepend={profile_picture.clone()}>
                {panda_card_content}
                <CardBottom slot>
                    <Button enabled={me_id != panda.id} label="Panda bannen" on:click=ban_panda_confirm />
                </CardBottom>
            </Card>
        }
    } else {
        view! {
            <Card title={display_name.clone()} prepend={profile_picture.clone()}>
                {panda_card_content}
            </Card>
        }
    }
}

#[component]
pub fn PandasList(#[prop(into, optional)] grove_id: Option<i32>) -> impl IntoView {
    let pandas = create_resource(|| (), move |_| async move { get_pandas(grove_id).await });

    let current_user = expect_context::<RwSignal<User>>();

    let refetch = {
        let pandas = pandas.clone();

        Callback::new(move |ban_result_code: BanResultCode| {
            if ban_result_code == BanResultCode::Success {
                pandas.refetch();
            }
        })
    };

    view! {
        <Transition fallback=move || view! { <ProgressRing /> }>
            {move || pandas.get().map(|pandas| {
                if let Ok(pandas) = pandas {
                    let current_user_id = current_user.get().id;
                    let is_mod = pandas
                                        .iter()
                                        .find(|panda| panda.id == current_user_id)
                                        .map(|panda| panda.is_mod)
                                        .unwrap_or(false);
                    let refetch = refetch.clone();

                    view! {
                        <CardList>
                            {move || pandas.iter().map(move |panda| {
                                let refetch = refetch.clone();

                                view! {
                                    <PandaCard ban_callback=refetch.clone() panda=panda.clone() grove_id=grove_id.clone() me_id=current_user_id is_mod=is_mod />
                                }
                            }).collect::<Vec<_>>()}
                        </CardList>
                    }
                } else {
                    view! {
                        <AlertMessage header="Fehler beim Laden" message_type=MessageType::Negative>
                            <MessageContent slot>
                                <p>Leider konnten die Pandas nicht geladen werden, wende dich bitte an den Bambusssupport.</p>
                            </MessageContent>
                        </AlertMessage>
                    }
                }
            })}
        </Transition>
    }
}
