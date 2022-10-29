use dioxus::prelude::*;

#[allow(non_snake_case,dead_code)]
#[inline_props]
pub fn PfDropDownRaw<'a>(
    cx: Scope<'a>,
    children: Element<'a>,
    selected: UseState<String>,
) -> Element {
    let is_hide = use_state(&cx, || true);
    let buttoncss = if *is_hide.get() {
        "pf-c-dropdown__toggle"
    } else {
        "pf-c-dropdown__toggle pf-m-expanded"
    };
    cx.render(rsx! {
        div { class: "pf-c-dropdown",
            button { class: "{buttoncss}", aria_expanded: "false",
                onclick: move |_| { is_hide.set(!is_hide.get()); },
                span { class: "pf-c-dropdown__toggle-text", "{selected}" },
                span { class: "pf-c-dropdown__toggle-icon",
                    i { class: "fas fa-angle-down", aria_hidden: "false" }
                }
            }
            div { class: "pf-c-dropdown__menu", hidden: "{is_hide}", onclick:  |_| {is_hide.set(!is_hide.get());},
                children
            }
        }
    })
}

#[allow(non_snake_case,dead_code)]
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

#[allow(non_snake_case,dead_code)]
#[inline_props]
pub fn PfDropDown(cx: Scope, list: Vec<String>, selected: UseState<String>) -> Element {
    let is_hide = use_state(&cx, || true);
    let buttoncss = if *is_hide.get() {
        "pf-c-dropdown__toggle"
    } else {
        "pf-c-dropdown__toggle pf-m-expanded"
    };
    cx.render(rsx! {
        div { class: "pf-c-dropdown",
            button { class: "{buttoncss}", aria_expanded: "false",
                onclick: move |_| { is_hide.set(!is_hide.get()); },
                span { class: "pf-c-dropdown__toggle-text", "{selected}" },
                span { class: "pf-c-dropdown__toggle-icon",
                    i { class: "fas fa-angle-down", aria_hidden: "false" }
                }
            }
            div { class: "pf-c-dropdown__menu", hidden: "{is_hide}",
                list.iter().map(|data|{ rsx!{
                    a { class: "pf-c-dropdown__menu-item", onclick: move |_| { selected.set(data.clone()); is_hide.set(!is_hide.get()); },
                        "{data}"
                    }
                }})
            }
        }
    })
}

#[allow(non_snake_case,dead_code)]
#[inline_props]
pub fn PfDropDownWithId(cx: Scope, list: Vec<(u64, String)>, selected: UseState<u64>) -> Element {
    let is_hide = use_state(&cx, || true);
    let selected_str = use_state(&cx, || "".to_string());
    let buttoncss = if *is_hide.get() {
        "pf-c-dropdown__toggle"
    } else {
        "pf-c-dropdown__toggle pf-m-expanded"
    };
    cx.render(rsx! {
        div { class: "pf-c-dropdown",
            button { class: "{buttoncss}",  aria_expanded: "false",
                onclick: move |_| { is_hide.set(!is_hide.get()); },
                span { class: "pf-c-dropdown__toggle-text", "{selected_str}" },
                span { class: "pf-c-dropdown__toggle-icon",
                    i { class: "fas fa-angle-down", aria_hidden: "false" }
                }
            }
            div { class: "pf-c-dropdown__menu", hidden: "{is_hide}",
            // list
            list.iter().map(|data|{ rsx!{
                a { class: "pf-c-dropdown__menu-item", onclick: move |_| { selected_str.set(data.1.clone()) ;selected.set(data.0); is_hide.set(!is_hide.get()); },
                    "{data.1}"
                }
            }}),
            }
        }
    })
}
