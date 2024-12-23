use leptos::prelude::*;
use leptos_cosmo::prelude::*;

#[slot]
pub struct CardBottom {
    #[prop(optional)]
    pub children: Option<Children>,
}

#[component]
pub fn Card(
    children: Children,
    #[prop(into)] title: Signal<String>,
    #[prop(into, default = "".into())] prepend: Signal<String>,
    #[prop(optional)] card_bottom: Option<CardBottom>,
) -> impl IntoView {
    view! {
        <div class="pandas-card">
            <div class="pandas-card__content" class:has--buttons=card_bottom.is_some()>
                {move || {
                    (!prepend.read().is_empty()).then_some(view! {
                                <div class="pandas-card__prepend">
                                    <img
                                        style="max-height:7rem;"
                                        src=prepend
                                    />
                                </div>
                            },
                        )
                }}
                <div class="pandas-card__content-text">
                    <h5>{title}</h5>
                    {children()}
                </div>
            </div>
            {card_bottom.map(|bottom| bottom.children.map(|bottom| view! { <ToolbarGroup>{bottom()}</ToolbarGroup> }))}
        </div>
    }
}

#[component]
pub fn CardList(children: Children) -> impl IntoView {
    view! { <div class="pandas-card-list">{children()}</div> }
}
