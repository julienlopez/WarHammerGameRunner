use dioxus::prelude::*;

use super::state::State;
use crate::ArmyRules;

#[component]
pub fn ArmySelector() -> Element {
    let mut state = use_context::<Signal<State>>();
    let armies = use_context::<Signal<ArmyRules>>()
        .read()
        .clone()
        .armies
        .into_iter()
        .enumerate()
        .map(|(index, army)| {
            let name = army.name.clone();
            rsx!( option{ value: "{index}", "{name}" })
        });
    let title_option =
        rsx!(option { value:"", disabled: true, selected: true, "--Please select an army--" });
    rsx!(
        form {
            onsubmit: move |event| {
                let values = event.data.values();
                if let (Some(p1_army), Some(p2_army)) = (values.get("player1_army"), values.get("player2_army")) {
                    *state.write() = State::DetachmentSelection { p1_army: p1_army.as_value().parse().unwrap(), p2_army: p2_army.as_value().parse().unwrap() };
                }
            },
            div {
                class: "main_screen",
                id: "army_selector",
                h1 { "Army Selection" },
                div {
                    id: "army_selection_panel",
                    div {
                        class: "army_selection_box",
                        h2 { "Player 1" }
                        select {
                            name: "player1_army",
                            {title_option.clone()},
                            {armies.clone()}
                        }
                    },
                    div {
                        class: "army_selection_box",
                        h2 { "Player 2" }
                        select {
                            name: "player2_army",
                            {title_option}
                            {armies}
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
