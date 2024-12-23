use crate::api::{get_all_groves, get_current_user, LogoutAction};
use crate::state::AllGroves;
use crate::{bamboo, final_fantasy, groves, my, support};
use bamboo_common::core::entities::User;
use leptos::prelude::*;
use leptos_cosmo::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;
use leptos_use::use_window;

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
            <SubMenu parent="/pandas/final-fantasy" slot>
                <MenuItem href="/pandas/final-fantasy" label="Charaktere" />
                <MenuItem href="/pandas/final-fantasy/customization" label="Personalisierung" />
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
        <PageBody>
            <Routes fallback=|| view! { <Redirect path="/pandas/bamboo" /> }>
                <Route path=path!("/pandas") view=|| view! { <Redirect path="/pandas/bamboo" /> } />
                <Route path=path!("/pandas/bamboo") view=bamboo::Calendar />
                <Route path=path!("/pandas/bamboo/pandas") view=bamboo::Pandas />
                <Route path=path!("/pandas/final-fantasy") view=final_fantasy::Characters />
                <Route path=path!("/pandas/groves/:id/:name") view=groves::GrovePage />
                <Route
                    path=path!("/pandas/groves")
                    view=move || {
                        let groves = groves.read();
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
    let current_user_ctx = expect_context::<RwSignal<User>>();

    let profile_picture =
        RwSignal::new(format!("/api/user/{}/picture", current_user_ctx.read().id));

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
    Effect::new(move |_| {
        *profile_picture.write() = format!("/api/user/{}/picture", current_user_ctx.read().id);
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

    let current_user_ctx = RwSignal::new(User::default());
    let groves_ctx = RwSignal::new(AllGroves::new());

    let current_user_resource = Resource::new(|| (), |_| async move { get_current_user().await });

    Effect::new(move |_| current_user_resource.refetch());

    provide_context(current_user_ctx);
    provide_context(groves_ctx);

    let groves_resource = Resource::new(|| (), |_| async move { get_all_groves().await });

    view! {
        <Stylesheet id="leptos" href="/pandas/pkg/frontend-pandas.css" />
        <Link href="/pandas/assets/favicon.svg" rel="icon" type_="image/svg+xml" />
        <Link href="/pandas/assets/favicon.png" rel="icon" type_="image/png" />

        <Link href="/pandas/assets/manifest.json" rel="manifest" />
        <Link href="/pandas/assets/favicon.svg" rel="mask-icon" />

        <Meta content="#598c79" name="msapplication-TileColor" />
        <Meta content="#598c79" name="theme-color" />
        <Meta content="width=device-width, initial-scale=1" name="viewport" />
        <Suspense>
            {move || Suspend::new(async move {
                if let Ok(groves) = groves_resource.await {
                    groves_ctx.set(groves);
                }
                if let Ok(current_user) = current_user_resource.await {
                    current_user_ctx.set(current_user)
                }
            })}
        </Suspense>

        <PageLayout
            primary_color=Color::new(89, 140, 121, 0.0)
            primary_color_dark=Color::new(89, 140, 121, 0.0)
        >
            <Router>
                <leptos_meta::Title formatter=|text| format!("{text} – Bambushain") />
                <PandasTopBar />
                <PandasMenu />
                <PandasRoutes />
            </Router>
        </PageLayout>
    }
}
