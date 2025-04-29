use dioxus::prelude::*;

use super::state::State;
use crate::ArmyRules;

#[component]
pub fn DetachmentSelector(p1_army_index: usize, p2_army_index: usize) -> Element {
    let mut state = use_context::<Signal<State>>();
    let rules = use_context::<Signal<ArmyRules>>().read().clone();
    let armies = &rules.armies;
    let p1_army = armies[p1_army_index].clone();
    let p1_detachments = rules.list_detachment_names_for_army(p1_army_index);
    let p1_detachments = p1_detachments
        .iter()
        .enumerate()
        .map(|(index, d)| rsx!( option{ value: "{index}", "{d}" }));
    let p2_army = armies[p2_army_index].clone();
    let p2_detachments = rules.list_detachment_names_for_army(p2_army_index);
    let p2_detachments = p2_detachments
        .iter()
        .enumerate()
        .map(|(index, d)| rsx!( option{ value: "{index}", "{d}" }));
    let title_option = rsx!(option { value:"", disabled: true, selected: true, "--Please select a detatchment--" });
    rsx!(
        form {
            onsubmit: move |event| {
                let values = event.data.values();
                if let (Some(p1_detachment), Some(p2_detachment)) = (values.get("player1_detachment"), values.get("player2_detachment")) {
                    *state.write() = State::Details {
                        player_1: (p1_army_index, p1_detachment.as_value().parse().unwrap()),
                        player_2: (p2_army_index, p2_detachment.as_value().parse().unwrap())
                    };
                }
            },
            div {
                class: "main_screen",
                id: "detachment_selector",
                h1 { "Detachment Selection" },
                div {
                    id: "detachment_selection_panel",
                    div {
                        class: "detachment_selection_box",
                        h2 { "Player 1" },
                        h2 { "{p1_army.name.clone()}" },
                        select {
                            name: "player1_detachment",
                            {title_option.clone()},
                            {p1_detachments}
                        }
                    },
                    div {
                        class: "detachment_selection_box",
                        h2 { "Player 2" }
                        h2 { "{p2_army.name.clone()}" },
                        select {
                            name: "player2_detachment",
                            {title_option}
                            {p2_detachments}
                        }
                    },
                }
                input {
                    class: "button-xlarge pure-button",
                    r#type: "submit",
                    "Next"
                }
            }
        }
    )
}
