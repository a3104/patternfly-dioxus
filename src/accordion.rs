use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn PfAccordion<'a>(
    cx: Scope<'a>,
    children: Element<'a>,
    title: &'a str,
    is_open: Option<bool>,
) -> Element {
    let default_open_flag = !is_open.unwrap_or(false);
    let is_close_accordion = use_state(&cx, || default_open_flag);
    let buttoncss = if *is_close_accordion.get() {
        "pf-c-accordion__toggle"
    } else {
        "pf-c-accordion__toggle pf-m-expanded"
    };
    cx.render(rsx! {
        div {class: "pf-c-accordion",
        h3 {
            button { class: "{buttoncss} ", r#type: "button", aria_expanded: "false",
            onclick: move |_| { is_close_accordion.set(!is_close_accordion.get()); },

                span { class: "pf-c-accordion__toggle-text", "{title}" },
                span { class: "pf-c-accordion__toggle-icon",
                    i { class: "fas fa-angle-right", aria_hidden: "false" }
                }
            }
        }
        div { class: "pf-c-accordion__expanded-content pf-m-expanded",
            div { class: "pf-c-accordion__expanded-content-body", hidden:"{is_close_accordion}" , children,  }
        }
    }
    })
}
