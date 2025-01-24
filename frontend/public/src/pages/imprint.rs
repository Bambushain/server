use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Imprint() -> impl IntoView {
    view! {
        <Title text="Impressum" />
        <h1>"Angaben gemäß § 5 TMG"</h1>
        <address>
            {"Imanuel Ulbricht"} <br /> {"Dingworthstr. 15"} <br /> {"31137 Hildesheim"}
        </address>
        <h2>"Kontakt"</h2>
        <p>
            {"E-Mail: "}
            <a href="mailto:panda.helferlein@bambushain.app">{"panda.helferlein@bambushain.app"}</a>
        </p>
        <h2>"Haftung für Inhalte"</h2>
        <p>
            {"Als Diensteanbieter sind wir gemäß § 7 Abs.1 TMG für eigene Inhalte auf diesen Seiten nach den allgemeinen Gesetzen verantwortlich. Nach §§ 8 bis 10 TMG sind wir als Diensteanbieter jedoch nicht verpflichtet, übermittelte oder gespeicherte fremde Informationen zu überwachen oder nach Umständen zu forschen, die auf eine rechtswidrige Tätigkeit hinweisen."}
        </p>
        <p>
            {"Verpflichtungen zur Entfernung oder Sperrung der Nutzung von Informationen nach den allgemeinen Gesetzen bleiben hiervon unberührt. Eine diesbezügliche Haftung ist jedoch erst ab dem Zeitpunkt der Kenntnis einer konkreten Rechtsverletzung möglich. Bei Bekanntwerden von entsprechenden Rechtsverletzungen werden wir diese Inhalte umgehend entfernen."}
        </p>
        <p>{"Quelle: e-recht24.de"}</p>
    }
}
