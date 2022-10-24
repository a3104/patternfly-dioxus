use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Variation {
    Default,
    Info,
    Success,
    Warning,
    Danger,
}

#[allow(non_snake_case)]
#[inline_props]
pub fn PfAlert<'a>(
    cx: Scope<'a>,
    variation: Option<Variation>,
    description: Option<&'a str>,
    children: Element<'a>,
) -> Element {
    let variation = variation.unwrap_or(Variation::Default);
    let css = match variation {
        Variation::Default => "pf-c-alert",
        Variation::Info => "pf-c-alert pf-m-info",
        Variation::Success => "pf-c-alert pf-m-success",
        Variation::Warning => "pf-c-alert pf-m-warning",
        Variation::Danger => "pf-c-alert pf-m-danger",
    };

    let iconcss = match variation {
        Variation::Default => "fas fa-exclamation-circle",
        Variation::Info => "fas fa-info-circle",
        Variation::Success => "fas fa-check-circle",
        Variation::Warning => "fas fa-exclamation-triangle",
        Variation::Danger => "fas fa-exclamation-circle",
    };

    cx.render(rsx! {
        div {
            class: "{css}",
            div { class: "pf-c-alert__icon",
                i { class: "{iconcss}",}
            },

            p { class: "pf-c-alert__title",
                span { class: "pf-screen-reader", "Default alert:" },
                children
            }
            {
                if let Some(description) = description {
                   rsx!{ p { class: "pf-c-alert__description", "{description}" }}
                } else {
                   rsx!("")
                }
            }
        },

    })
}
