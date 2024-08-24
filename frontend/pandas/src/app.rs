use crate::api::get_current_user;
use crate::bamboo;
use bamboo_common::core::entities::User;
use leptos::*;
use leptos_cosmo::prelude::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let current_user = create_rw_signal(User::default());

    let load_current_user =
        create_blocking_resource(|| {}, |_| async move { get_current_user().await });

    create_effect(move |_| load_current_user.refetch());

    provide_context(current_user);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pandas/pkg/frontend-pandas.css"/>
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
                        load_current_user.get().map(|user| {
                            if let Ok(user) = user {
                                current_user.set(user.clone());
                                Some(view! {
                                    <TopBar has_right_item=true right_item_label="Abmelden" profile_picture={format!("/api/user/{}/picture", user.id)}>
                                        <TopBarItem label="Lizenzen" />
                                        <TopBarItem label="Impressum" />
                                        <TopBarItem label="Datenschutz" />
                                    </TopBar>
                                })
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
                    </SubMenu>
                </Menu>
                <PageBody>
                    <Routes>
                        <Route path="/pandas" view=|| view! { <Redirect path="/pandas/bamboo" /> } />
                        <Route path="/pandas/bamboo" view=bamboo::Calendar />
                    </Routes>
                </PageBody>
            </Router>
        </PageLayout>
    }
}
