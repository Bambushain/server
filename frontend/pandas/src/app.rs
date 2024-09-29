use crate::api::{get_all_groves, get_current_user};
use crate::{bamboo, groves};
use bamboo_common::core::entities::{Grove, User};
use leptos::*;
use leptos_cosmo::prelude::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let current_user = create_rw_signal(User::default());
    let groves_signal = create_rw_signal(Vec::<Grove>::new());

    let load_current_user =
        create_blocking_resource(|| {}, |_| async move { get_current_user().await });

    create_effect(move |_| load_current_user.refetch());

    provide_context(current_user);
    provide_context(groves_signal);

    let load_groves = create_blocking_resource(|| {}, |_| async move { get_all_groves().await });

    create_effect(move |_| {
        load_groves.refetch();
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
                <Transition>
                    {move || {
                        load_groves
                            .get()
                            .map(|groves| {
                                if let Ok(groves) = groves {
                                    groves_signal.set(groves.to_owned());
                                }
                            })
                    }}
                    {move || {
                        load_current_user
                            .get()
                            .map(|user| {
                                if let Ok(user) = user {
                                    current_user.set(user.clone());
                                    Some(
                                        view! {
                                            <TopBar
                                                has_right_item=true
                                                right_item_label="Abmelden"
                                                profile_picture=format!("/api/user/{}/picture", user.id)
                                            >
                                                <TopBarItem label="Lizenzen" />
                                                <TopBarItem label="Impressum" />
                                                <TopBarItem label="Datenschutz" />
                                            </TopBar>
                                        },
                                    )
                                } else {
                                    None
                                }
                            })
                    }}
                </Transition>
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
                        <Transition>
                            {move || {
                                load_groves
                                    .get()
                                    .map(|groves| {
                                        groves
                                            .map(|groves| {
                                                groves
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
                                            })
                                            .ok()
                                    })
                            }}
                        </Transition>
                        <MenuItem href="/pandas/groves/new" label="Neuer Hain" />
                    </SubMenu>
                </Menu>
                <PageBody>
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
                                let groves = groves_signal.get();
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
                    </Routes>
                </PageBody>
            </Router>
        </PageLayout>
    }
}
