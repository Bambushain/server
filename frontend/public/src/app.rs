use crate::pages;
use chrono::Datelike;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/public/pkg/frontend-public.css" />
        <Link href="/public/assets/favicon.svg" rel="icon" type_="image/svg+xml" />
        <Link href="/public/assets/favicon.png" rel="icon" type_="image/png" />

        <Link href="/public/assets/manifest.json" rel="manifest" />
        <Link href="/public/assets/favicon.svg" rel="mask-icon" />

        <Meta content="#598c79" name="msapplication-TileColor" />
        <Meta content="#598c79" name="theme-color" />
        <Meta content="width=device-width, initial-scale=1" name="viewport" />
        <Title formatter=|text| format!("{text} – Bambushain") />
        <Router>
            <main>
                <nav>
                    <span>Bambushain</span>
                </nav>
                <div>
                    <img src="/public/assets/background.webp" />
                </div>
                    <Routes fallback=|| view! { <Redirect path="/pandas" /> }>
                        <Route path=path!("/legal/licenses") view=pages::Licenses />
                        <Route path=path!("/legal/imprint") view=pages::Imprint />
                        <Route path=path!("/legal/privacy") view=pages::Privacy />
                        <Route path=path!("/*any") view=|| view! { <Redirect path="/pandas" /> }/>
                    </Routes>
                <footer>
                    <div>{format!("© {}", chrono::Local::now().year())}</div>
                    <div>"Made with "<span>"❤ "</span>"in Hildesheim"</div>
                    <div>
                        <A href="/legal/licenses">Lizenzen</A>
                        <A href="/legal/imprint">Impressum</A>
                        <A href="/legal/privacy">Datenschutz</A>
                    </div>
                </footer>
            </main>
        </Router>
    }
}
