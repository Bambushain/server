use chrono::Datelike;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn BannerPage(#[prop(into)] title: String, children: Children) -> impl IntoView {
    view! {
        <Title text=title.clone() />
        <div class="bamboo-page">
            <div class="bamboo-banner__container">
                <img class="bamboo-banner" src="/public/assets/background.webp" />
                <span class="bamboo-banner__title">{title}</span>
            </div>
            <main class="bamboo-page__content">
                {children()}
            </main>
            <footer class="bamboo-footer">
                <div>{format!("© Bambushain Team {}", chrono::Local::now().year())}</div>
                <div>"Made with "<span class="bamboo-heart">"❤ "</span>"in Hildesheim"</div>
                <div class="bamboo-footer__links">
                    <A href="/legal/licenses">"Lizenzen"</A>
                    <A href="/legal/imprint">"Impressum"</A>
                    <A href="/legal/privacy">"Datenschutz"</A>
                </div>
            </footer>
        </div>
    }
}
