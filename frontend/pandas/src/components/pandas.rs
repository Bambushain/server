use crate::api::{get_pandas, BanPandaAction, BanResultCode};
use crate::components::{Card, CardBottom, CardList};
use bamboo_common::core::entities::user::GroveUser;
use bamboo_common::core::entities::BambooUser;
use leptos::either::Either;
use leptos::prelude::*;
use leptos_cosmo::prelude::*;

#[component]
fn PandaCard(
    panda: GroveUser,
    #[prop(into)] grove_id: Signal<Option<i32>>,
    #[prop(into)] me_id: Signal<i32>,
    #[prop(into)] is_mod: Signal<Option<bool>>,
    #[prop(into)] ban_callback: Callback<BanResultCode>,
) -> impl IntoView {
    let profile_picture = Memo::new(move |_| {
        format!(
            "/api/user/{}/picture#time={}",
            panda.id,
            Local::now().timestamp_millis()
        )
    });
    let display_name = Memo::new(move |_| panda.display_name.clone());
    let discord_name = Memo::new(move |_| panda.discord_name.clone());
    let email = Memo::new(move |_| panda.email.clone());

    let ban_panda_action = ServerAction::<BanPandaAction>::new();

    let is_me = Memo::new(move |_| me_id.get() != panda.id);

    let ban_panda_confirm = {
        let user_id = panda.id;

        move |_| {
            let grove_id = grove_id.get().unwrap();

            use_modals().confirm(
                "Panda bannen",
                format!(
                    "Soll der Panda {} wirklich gebannt werden?",
                    display_name.read()
                ),
                Variant::Negative,
                format!("{} bannen", display_name.read()),
                format!("{} nicht bannen", display_name.read()),
                Some(Callback::new(move |_| {
                    ban_panda_action.dispatch(BanPandaAction { grove_id, user_id });
                })),
                None,
            )
        }
    };

    let panda_card_content = move || {
        view! {
            <a href=format!("mailto:{}", email.read())>{email}</a>
            <Show when=move || !discord_name.read().is_empty()>
                <span>{"Auf Discord bekannt als "}<strong>{discord_name}</strong></span>
            </Show>
        }
    };

    Effect::new(move |_| {
        if let Some(Ok(result)) = ban_panda_action.value().get() {
            ban_callback.run(result)
        }
    });

    view! {
        <Show
            when=move || grove_id.read().is_some() && is_mod.get().unwrap_or(false)
            fallback=move || {
                view! {
                    <Card title=display_name prepend=profile_picture>
                        {panda_card_content()}
                    </Card>
                }
            }
        >
            <Card title=display_name prepend=profile_picture>
                {panda_card_content()}
                <CardBottom slot>
                    <Button enabled=is_me label="Panda bannen" on:click=ban_panda_confirm />
                </CardBottom>
            </Card>
        </Show>
    }
}

#[component]
pub fn PandasList(#[prop(into, optional)] grove_id: Signal<Option<i32>>) -> impl IntoView {
    let pandas = Resource::new(
        move || grove_id.get(),
        move |grove_id| async move { get_pandas(grove_id).await },
    );

    let current_user = expect_context::<RwSignal<BambooUser>>();

    let refetch = Callback::new(move |ban_result_code: BanResultCode| {
        if ban_result_code == BanResultCode::Success {
            pandas.refetch();
        }
    });

    view! {
        <Transition fallback=|| {
            view! { <ProgressRing /> }
        }>
            {move || {
                Suspend::new(async move {
                    pandas
                        .await
                        .ok()
                        .map(|pandas| {
                            let current_user_id = current_user.get().id;
                            let is_mod = pandas
                                .iter()
                                .find(|panda| panda.id == current_user_id)
                                .map(|panda| panda.is_mod)
                                .unwrap_or(false);
                            Either::Left(
                                view! {
                                    <CardList>
                                        {move || {
                                            pandas
                                                .iter()
                                                .map(move |panda| {
                                                    view! {
                                                        <PandaCard
                                                            ban_callback=refetch
                                                            panda=panda.clone()
                                                            grove_id=grove_id
                                                            me_id=current_user_id
                                                            is_mod=is_mod
                                                        />
                                                    }
                                                })
                                                .collect::<Vec<_>>()
                                        }}
                                    </CardList>
                                },
                            )
                        })
                        .unwrap_or(
                            Either::Right(
                                view! {
                                    <AlertMessage
                                        header="Fehler beim Laden"
                                        message_type=MessageType::Negative
                                    >
                                        <MessageContent slot>
                                            <p>
                                                Leider konnten die Pandas nicht geladen werden, wende dich bitte an den Bambusssupport.
                                            </p>
                                        </MessageContent>
                                    </AlertMessage>
                                },
                            ),
                        )
                })
            }}
        </Transition>
    }
}
