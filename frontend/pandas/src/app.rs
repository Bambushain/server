use crate::api::{get_all_groves, get_current_user};
use crate::state::AllGroves;
use crate::{bamboo, groves};
use bamboo_common::core::entities::User;
use leptos::*;
use leptos_cosmo::prelude::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
fn PandasMenu() -> impl IntoView {
    let groves = expect_context::<RwSignal<AllGroves>>();

    view! {
        <Menu>
            <MainMenu slot>
                <MenuItem main=true href="/pandas/bamboo" label="Bambushain" />
                <MenuItem main=true href="/pandas/final-fantasy" label="Final Fantasy" />
                <MenuItem main=true href="/pandas/groves" label="Meine Haine" />
                <MenuItem main=true href="/pandas/profile" label="Mein Profil" />
                <MenuItem main=true href="/pandas/support" label="Bambussupport" />
            </MainMenu>
            <SubMenu parent="/pandas/bamboo" slot>
                <MenuItem href="/pandas/bamboo" label="Event Kalender" />
                <MenuItem href="/pandas/bamboo/pandas" label="Pandas" />
            </SubMenu>
            <SubMenu parent="/pandas/groves" slot>
                {move || {
                    groves
                        .get()
                        .iter()
                        .map(|grove| {
                            view! {
                                <MenuItem
                                    href=format!("/pandas/groves/{}/{}", grove.id, grove.name)
                                    label=grove.name.clone()
                                />
                            }
                        })
                        .collect_view()
                }}
                <MenuItem href="/pandas/groves/new" label="Neuer Hain" />
            </SubMenu>
        </Menu>
    }
}

#[component]
fn PandasRoutes() -> impl IntoView {
    let groves = expect_context::<RwSignal<AllGroves>>();

    view! {
        <Routes>
            <Route
                path="/pandas"
                view=|| view! { <Redirect path="/pandas/bamboo" /> }
            />
            <Route path="/pandas/bamboo" view=bamboo::Calendar />
            <Route path="/pandas/bamboo/pandas" view=bamboo::Pandas />
            <Route path="/pandas/groves/:id/:name" view=groves::GrovePage />
            <Route
                path="/pandas/groves"
                view=move || {
                    let groves = groves.get();
                    if let Some(grove) = groves.first() {
                        view! {
                            <Redirect path=format!(
                                "/pandas/groves/{}/{}",
                                grove.id,
                                grove.name.clone(),
                            ) />
                        }
                    } else {
                        view! { <Redirect path="/pandas/groves/new" /> }
                    }
                }
            />
            <Route path="/pandas/groves/new" view=groves::NewGrovePage />
        </Routes>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let current_user_ctx = create_rw_signal(User::default());
    let groves_ctx = create_rw_signal(AllGroves::new());

    let profile_picture =
        create_memo(move |_| format!("/api/user/{}/picture", current_user_ctx.get().id));

    let current_user_resource =
        create_local_resource(|| {}, |_| async move { get_current_user().await });

    create_effect(move |_| current_user_resource.refetch());

    provide_context(current_user_ctx);
    provide_context(groves_ctx);

    let groves_resource = create_local_resource(|| {}, |_| async move { get_all_groves().await });

    create_effect(move |_| {
        if let Some(Ok(groves)) = groves_resource.get() {
            groves_ctx.set(groves);
        }
    });
    create_effect(move |_| {
        if let Some(Ok(current_user)) = current_user_resource.get() {
            current_user_ctx.set(current_user)
        }
    });
    create_effect(move |_| {
        groves_resource.refetch();
    });

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
                <TopBar
                    has_right_item=true
                    right_item_label="Abmelden"
                    profile_picture=profile_picture
                >
                    <TopBarItem label="Lizenzen" />
                    <TopBarItem label="Impressum" />
                    <TopBarItem label="Datenschutz" />
                </TopBar>
                <PandasMenu />
                <PageBody>
                    <PandasRoutes />
                </PageBody>
            </Router>
        </PageLayout>
    }
}
