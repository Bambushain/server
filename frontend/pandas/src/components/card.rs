use leptos::*;
use leptos_cosmo::prelude::*;

#[slot]
pub struct CardBottom {
    #[prop(optional)]
    pub children: Option<Children>,
}

#[component]
pub fn Card(
    children: Children,
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into, default = "".into())] prepend: MaybeSignal<String>,
    #[prop(optional)] card_bottom: Option<CardBottom>,
) -> impl IntoView {
    let card_content_class = if card_bottom.is_some() {
        "pandas-card__content has--buttons"
    } else {
        "pandas-card__content"
    };

    view! {
        <div class="pandas-card">
            <div class=card_content_class>
                {move || {
                    if !prepend.get().is_empty() {
                        Some(
                            view! {
                                <div class="pandas-card__prepend">
                                    <img
                                        style="max-height:7rem;"
                                        src={
                                            let prepend = prepend.clone();
                                            move || prepend.get()
                                        }
                                    />
                                </div>
                            },
                        )
                    } else {
                        None
                    }
                }} <div class="pandas-card__content-text">
                    <h5>{title}</h5>
                    {children()}
                </div>
            </div>
            {if let Some(Some(children)) = card_bottom.map(|bottom| bottom.children) {
                {
                    view! { <ToolbarGroup>{children().into_view()}</ToolbarGroup> }
                }
            } else {
                ().into_view()
            }}
        </div>
    }
}

#[component]
pub fn CardList(children: Children) -> impl IntoView {
    view! { <div class="pandas-card-list">{children()}</div> }
}
