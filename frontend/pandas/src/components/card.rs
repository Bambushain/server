use leptos::*;
use leptos_cosmo::prelude::*;

#[slot]
pub struct Card {
    pub children: ChildrenFn,
    #[prop(into)]
    pub title: MaybeSignal<String>,
    #[prop(into, optional)]
    pub prepend: Option<ChildrenFn>,
    #[prop(into, optional)]
    pub buttons: Option<ChildrenFn>,
}

impl Card {
    fn render(&self) -> impl IntoView {
        let buttons = self.buttons.clone();
        let prepend = self.prepend.clone();

        view! {
            <div class="panda-card">
                <div class="panda-card__content">
                    {move || prepend.clone().map(|prepend| view! {
                        <div class="panda-card__prepend">{prepend}</div>
                    })}
                    <div class="panda-card__content-text">
                        <h1>{self.title.clone()}</h1>
                        {(self.children)()}
                    </div>
                </div>
                {move || buttons.clone().map(|buttons| view! {
                    <ToolbarGroup>
                        {buttons}
                    </ToolbarGroup>
                })}
            </div>
        }
    }
}

#[component]
pub fn CardList(#[prop(into)] cards: Vec<Card>) -> impl IntoView {
    view! {
        <div class="panda-card-list">
            {cards.iter().map(Card::render).collect::<Vec<_>>()}
        </div>
    }
}
