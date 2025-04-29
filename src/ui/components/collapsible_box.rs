use dioxus::prelude::*;

#[component]
pub fn CollapsibleBox(header: Element, body: Element, style: Option<String>) -> Element {
    let mut collapsed = use_signal(|| true);
    rsx!(
        div{
            style: style.unwrap_or("".to_string()),
            div{
                onclick: move |_| {
                    let current = *collapsed.clone().read();
                    *collapsed.write() = !(current);
                },
                { header }
            }
            div {
                style: format!("display: {};", if *collapsed.read() { "none" } else { "block" }),
                { body }
            }
        }
    )
}
