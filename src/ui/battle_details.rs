use dioxus::prelude::*;

// use super::state::State;
use crate::army_rules::{Enhancement, Key, Stratagem};
use crate::ArmyRules;

use super::components::collapsible_box::CollapsibleBox;

#[component]
pub fn BattleDetails(player_1: (usize, usize), player_2: (usize, usize)) -> Element {
    rsx!(
        div {
            class: "main_screen",
            id: "detachment_selector",
            h1 { "Armies Details" },
            div {
                id: "detachment_selection_panel",
                DetachmentDetails {
                    title: "Player 1".to_string(), player: player_1
                },
                DetachmentDetails {
                    title: "Player 2".to_string(), player: player_2
                },
            }
            div {
                onclick: |_| {},
                class: "button-xlarge pure-button",
                "Start Battle"
            }
        }
    )
}

fn stratagem_color(key: &Key) -> &'static str {
    match key {
        Key::EitherPlayersTurn => "#1E6257",
        Key::YourTurn => "#0F4D6B",
        Key::OpponentsTurn => "#A21217",
    }
}

#[component]
fn DetachmentDetails(title: String, player: (usize, usize)) -> Element {
    let rules = use_context::<Signal<ArmyRules>>().read().clone();
    let army = rules.armies[player.0].clone();
    let detachment = rules.detachments[player.1].clone();
    rsx!(
        div {
            class: "detachment_selection_box",
            h2 { title },
            h2 { "{army.name}" },
            h2 { "{detachment.name}" },
            h3 { "Enhancements" },
            div{
                for e in detachment.enhancements {
                    EnhancementBox{e: e.clone()}
                }
           },
            h3 { "Stratagems" },
            div{
                for s in detachment.stratagems {
                    StratagemBox{s: s.clone()}
                }
            },
            h3 { "Detachment rules" }
            div{}
        },
    )
}

#[component]
fn EnhancementBox(e: Enhancement) -> Element {
    rsx!(CollapsibleBox {
        header: rsx!(div {
            class: "enhancement_box",
            {e.name}
            span {
                class: "enhancement_cost_box",
                { format!(" {} pts", e.cost) }
            }
        }),
        body: rsx!(
            div {
                style: "padding: 5px;",
                "plop"
        })
    })
}

#[component]
fn StratagemBox(s: Stratagem) -> Element {
    rsx!(CollapsibleBox {
        header: rsx!(div {
            class: "stratagem_box",
            style: format!("background-color: {};", stratagem_color(&s.key)),
            {s.name}
            span {
                class: "stratagem_cost_box",
                {format!("{} CP", s.cp_cost)}
            }
        }),
        body: rsx!(
            div {
                style: "padding: 5px;",
                "plop"
        }),
        style: "border-radius: 5px; background-color: #cac6bb; margin-bottom: 5px;"
    })
}
