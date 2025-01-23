#![recursion_limit = "1024"]

use crate::api::{get_all_groves, get_current_user, LogoutAction};
use crate::state::AllGroves;
use crate::{bamboo, final_fantasy, groves, my, support};
use bamboo_common::core::entities::BambooUser;
use leptos::prelude::*;
use leptos_cosmo::icons::*;
use leptos_cosmo::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::hooks::use_location;
use leptos_router::path;
use leptos_use::use_window;
use std::future::IntoFuture;

#[component]
fn PandasMenu() -> impl IntoView {
    let groves_ctx = expect_context::<RwSignal<AllGroves>>();
    let current_user_ctx = expect_context::<RwSignal<BambooUser>>();

    let logout_action = ServerAction::<LogoutAction>::new();

    let logout = move |_| {
        logout_action.dispatch(LogoutAction {});
    };

    let router = use_location();
    let profile_picture = Memo::new(move |_| current_user_ctx.get().profile_picture);

    Effect::new(move |_| {
        if logout_action.value().get().is_some_and(|res| res.is_ok()) {
            let window = use_window();
            let _ = window
                .as_ref()
                .unwrap()
                .location()
                .set_href("/authentication");
        }
    });

    view! {
        <div class="cosmo-menu is--bamboo">
            <img class="pandas-profilepicture" src=profile_picture aria-hidden />
            <nav class="cosmo-menu__collection is--bamboo">
                <div class="cosmo-menu__row is--main">
                    <MenuItem main=true href="/pandas/bamboo" label="Bambushain" />
                    <MenuItem main=true href="/pandas/final-fantasy" label="Final Fantasy" />
                    <MenuItem main=true href="/pandas/groves" label="Meine Haine" />
                    <MenuItem main=true href="/pandas/profile" label="Mein Profil" />
                    <MenuItem main=true href="/pandas/support" label="Bambussupport" />
                </div>
                <div class="cosmo-menu__row is--sub">
                    <Show when=move || { router.pathname.read().starts_with("/pandas/bamboo") }>
                        <MenuItem href="/pandas/bamboo" label="Event Kalender" />
                        <MenuItem href="/pandas/bamboo/pandas" label="Pandas" />
                    </Show>
                    <Show when=move || {
                        router.pathname.read().starts_with("/pandas/final-fantasy")
                    }>
                        <MenuItem href="/pandas/final-fantasy" label="Charaktere" />
                        <MenuItem
                            href="/pandas/final-fantasy/free-companies"
                            label="Freie Gesellschaften"
                        />
                        <MenuItem
                            href="/pandas/final-fantasy/custom-fields"
                            label="Eigene Felder"
                        />
                    </Show>
                    <Show when=move || {
                        router.pathname.read().starts_with("/pandas/groves")
                    }>
                        {move || {
                            groves_ctx
                                .get()
                                .into_iter()
                                .map(|grove| {
                                    view! {
                                        <MenuItem
                                            href=format!(
                                                "/pandas/groves/{}/{}",
                                                grove.id,
                                                grove.name.clone(),
                                            )
                                            label=grove.name.clone()
                                        />
                                    }
                                })
                                .collect_view()
                        }} <MenuItem href="/pandas/groves/new" label="Neuer Hain" />
                    </Show>
                </div>
            </nav>
            <button on:click=logout class="pandas-logout" aria-label="Abmelden">
                <Icon icon=LuLogOut width="2rem" height="2rem" />
            </button>
        </div>
    }
}

#[component]
fn GrovesRoute() -> impl IntoView {
    let groves = expect_context::<RwSignal<AllGroves>>();

    let groves = groves.read();
    if let Some(grove) = groves.first() {
        view! { <Redirect path=format!("/pandas/groves/{}/{}", grove.id, grove.name.clone()) /> }
    } else {
        view! { <Redirect path="/pandas/groves/new" /> }
    }
}

#[component]
fn PandasRoutes() -> impl IntoView {
    view! {
        <PageBody>
            <Routes fallback=|| view! { <Redirect path="/pandas/bamboo" /> }>
                <Route path=path!("/pandas") view=|| view! { <Redirect path="/pandas/bamboo" /> } />
                <Route path=path!("/pandas/bamboo") view=bamboo::Calendar />
                <Route path=path!("/pandas/bamboo/pandas") view=bamboo::Pandas />
                <Route
                    path=path!("/pandas/final-fantasy/free-companies")
                    view=final_fantasy::FreeCompanies
                />
                <Route
                    path=path!("/pandas/final-fantasy/custom-fields")
                    view=final_fantasy::CustomFields
                />
                <Route path=path!("/pandas/final-fantasy") view=final_fantasy::Characters />
                <Route path=path!("/pandas/groves/:id/:name") view=groves::GrovePage />
                <Route path=path!("/pandas/groves") view=GrovesRoute />
                <Route path=path!("/pandas/groves/new") view=groves::NewGrovePage />
                <Route path=path!("/pandas/profile") view=my::MyProfilePage />
                <Route path=path!("/pandas/support") view=support::BambooSupportPage />
            </Routes>
        </PageBody>
    }
}

#[component]
fn PandasTopBar() -> impl IntoView {
    let logout_action = ServerAction::<LogoutAction>::new();
    let current_user_ctx = expect_context::<RwSignal<BambooUser>>();

    let profile_picture = Memo::new(move |_| current_user_ctx.get().profile_picture);

    let logout = Callback::new(move |_| {
        logout_action.dispatch(LogoutAction {});
    });

    Effect::new(move |_| {
        if logout_action.value().get().is_some_and(|res| res.is_ok()) {
            let window = use_window();
            let _ = window
                .as_ref()
                .unwrap()
                .location()
                .set_href("/authentication");
        }
    });

    view! {
        <TopBar
            has_right_item=true
            right_item_label="Abmelden"
            right_item_on_click=logout
            profile_picture=profile_picture
        >
            <TopBarItem label="Lizenzen" />
            <TopBarItem label="Impressum" />
            <TopBarItem label="Datenschutz" />
        </TopBar>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let current_user_ctx = RwSignal::new(BambooUser::default());
    let groves_ctx = RwSignal::new(AllGroves::new());

    let groves_resource = Resource::new(|| (), move |_| async move { get_all_groves().await });
    let profile_resource = Resource::new(|| (), move |_| async move { get_current_user().await });

    provide_context(current_user_ctx);
    provide_context(groves_ctx);

    view! {
        <Stylesheet id="leptos" href="/pandas/pkg/frontend-pandas.css" />
        <Link href="/pandas/assets/favicon.svg" rel="icon" type_="image/svg+xml" />
        <Link href="/pandas/assets/favicon.png" rel="icon" type_="image/png" />

        <Link href="/pandas/assets/manifest.json" rel="manifest" />
        <Link href="/pandas/assets/favicon.svg" rel="mask-icon" />

        <Meta content="#598c79" name="msapplication-TileColor" />
        <Meta content="#598c79" name="theme-color" />
        <Meta content="width=device-width, initial-scale=1" name="viewport" />
        <PageLayout
            primary_color=Color::new(89, 140, 121, 0.0)
            primary_color_dark=Color::new(89, 140, 121, 0.0)
        >
            <Router>
                <leptos_meta::Title formatter=|text| format!("{text} – Bambushain") />
                <Transition>
                    {move || Suspend::new(async move {
                        if let (Ok(groves), Ok(profile)) = futures_util::join!(
                            groves_resource.into_future(), profile_resource.into_future()
                        ) {
                            groves_ctx.set(groves.clone());
                            current_user_ctx.set(profile);
                        }

                        view! { <PandasMenu /> }
                    })}
                </Transition>
                <PandasRoutes />
            </Router>
            <div class="cosmo-bottom-bar">
                <div class="cosmo-bottom-bar__item is--left">
                    <a>Lizenzen</a>
                    <a>Impressum</a>
                    <a>Datenschutz</a>
                </div>
            </div>
        </PageLayout>
    }
}
