use dioxus::{prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
}

#[allow(non_snake_case)]
#[inline_props]
pub fn PfTooltip<'a>(
    cx: Scope,
    children: Element<'a>,
    content: &'a str,
    orientation: Option<Orientation>,
) -> Element {
    let is_close = use_state(&cx, || true);
    let orientation = orientation.unwrap_or(Orientation::Top);
    let css = match orientation {
        Orientation::Top => "pf-c-tooltip pf-m-top",
        Orientation::Bottom => "pf-c-tooltip pf-m-bottom",
        Orientation::Left => "pf-c-tooltip pf-m-left",
        Orientation::Right => "pf-c-tooltip pf-m-right",
    };
    cx.render(rsx! {
        div { onmouseover: move |_| { is_close.set(false); }, onmouseout: move |_| { is_close.set(true); },
            div { class: "{css}", role: "tooltip",hidden: "{is_close}",
                div { class: "pf-c-tooltip__arrow",  },
                div { class: "pf-c-tooltip__content",
                    "{content}",
                }
            }
            ,children
        }
    })
}
