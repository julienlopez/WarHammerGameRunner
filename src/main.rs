use dioxus::prelude::*;

use ui::battle_details::BattleDetails;

pub mod army_rules;
use crate::army_rules::{generate_army_rules, ArmyRules};

pub mod ui;
use crate::ui::state::State;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(State::NotStarted));
    use_context_provider(|| Signal::new(generate_army_rules()));
    rsx! {
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/purecss@3.0.0/build/pure-min.css",
            integrity: "sha384-X38yfunGUhNzHpBaEBsWLO+A0HDYOQi8ufWDkZ0k9e0eXz/tH3II7uKZ9msv++Ls",
            crossorigin: "anonymous",
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    match *use_context::<Signal<State>>().read() {
        State::NotStarted => {
            rsx! {
                StartingScreen {}
            }
        }
        State::ArmySelection => {
            rsx! {
                crate::ui::army_selector::ArmySelector {}
            }
        }
        State::DetachmentSelection { p1_army, p2_army } => {
            rsx!(crate::ui::detachment_selector::DetachmentSelector {
                p1_army_index: p1_army,
                p2_army_index: p2_army
            })
        }
        State::Details { player_1, player_2 } => {
            rsx!(crate::ui::battle_details::BattleDetails { player_1, player_2 })
        }
    }
}

#[component]
fn StartingScreen() -> Element {
    rsx!(
        div { class: "main_screen", id: "starting_screen",
            div {
                id: "start_game_button",
                class: "button-xlarge pure-button",
                onclick: |_| {
                    *use_context::<Signal<State>>().write() = State::ArmySelection;
                },
                "Start new game!"
            }
        }
    )
}
