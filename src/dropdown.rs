use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn PfDropDownRaw<'a>(
    cx: Scope<'a>,
    children: Element<'a>,
    selected: UseState<String>,
) -> Element {
    let is_open = use_state(&cx, || false);
    let buttoncss = if *is_open.get() {
        "pf-c-dropdown__toggle"
    } else {
        "pf-c-dropdown__toggle pf-m-expanded"
    };
    cx.render(rsx! {
        div { class: "pf-c-dropdown",
            button { class: "{buttoncss}", aria_expanded: "false",
                onclick: move |_| { is_open.set(!is_open.get()); },
                span { class: "pf-c-dropdown__toggle-text", "{selected}" },
                span { class: "pf-c-dropdown__toggle-icon",
                    i { class: "fas fa-angle-down", aria_hidden: "false" }
                }
            }
            div { class: "pf-c-dropdown__menu", hidden: "{is_open}", onclick:  |_| {is_open.set(!is_open.get());},
                children
            }
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
pub fn PfDropDownItem<'a>(
    cx: Scope<'a>,
    children: Element<'a>,
    item_str: String,
    selected: UseState<String>,
) -> Element {
    cx.render(rsx! {
        span { class: "pf-c-dropdown__menu-item",
            onclick: move |_| { selected.set(item_str.clone()) },
            children
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
pub fn PfDropDown(cx: Scope, list: Vec<String>, selected: UseState<String>) -> Element {
    let is_open = use_state(&cx, || false);
    let buttoncss = if *is_open.get() {
        "pf-c-dropdown__toggle"
    } else {
        "pf-c-dropdown__toggle pf-m-expanded"
    };
    cx.render(rsx! {
        div { class: "pf-c-dropdown",
            button { class: "{buttoncss}", aria_expanded: "false",
                onclick: move |_| { is_open.set(!is_open.get()); },
                span { class: "pf-c-dropdown__toggle-text", "{selected}" },
                span { class: "pf-c-dropdown__toggle-icon",
                    i { class: "fas fa-angle-down", aria_hidden: "false" }
                }
            }
            div { class: "pf-c-dropdown__menu", hidden: "{is_open}",
                list.iter().map(|data|{ rsx!{
                    a { class: "pf-c-dropdown__menu-item", onclick: move |_| { selected.set(data.clone()); is_open.set(!is_open.get()); },
                        "{data}"
                    }
                }})
            }
        }
    })
}

#[allow(non_snake_case)]
#[inline_props]
pub fn PfDropDownWithId(cx: Scope, list: Vec<(u64, String)>, selected: UseState<u64>) -> Element {
    let is_open = use_state(&cx, || false);
    let selected_str = use_state(&cx, || "".to_string());
    let buttoncss = if *is_open.get() {
        "pf-c-dropdown__toggle"
    } else {
        "pf-c-dropdown__toggle pf-m-expanded"
    };
    cx.render(rsx! {
        div { class: "pf-c-dropdown",
            button { class: "{buttoncss}",  aria_expanded: "false",
                onclick: move |_| { is_open.set(!is_open.get()); },
                span { class: "pf-c-dropdown__toggle-text", "{selected_str}" },
                span { class: "pf-c-dropdown__toggle-icon",
                    i { class: "fas fa-angle-down", aria_hidden: "false" }
                }
            }
            div { class: "pf-c-dropdown__menu", hidden: "{is_open}",
            // list
            list.iter().map(|data|{ rsx!{
                a { class: "pf-c-dropdown__menu-item", onclick: move |_| { selected_str.set(data.1.clone()) ;selected.set(data.0); is_open.set(!is_open.get()); },
                    "{data.1}"
                }
            }}),
            }
        }
    })
}
