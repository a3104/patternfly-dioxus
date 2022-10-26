use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn PfCard<'a>(cx: Scope<'a>, element: Element<'a>) -> Element {
    cx.render(rsx! {
        div { class: "pf-c-card", element }
    })
}

#[allow(non_snake_case)]
#[inline_props]
pub fn PfCardTitle<'a>(cx: Scope<'a>, element: Element<'a>) -> Element {
    cx.render(rsx! {
        div { class: "pf-c-card__title", element }
    })
}

#[allow(non_snake_case)]
#[inline_props]
pub fn PfCardBody<'a>(cx: Scope<'a>, element: Element<'a>) -> Element {
    cx.render(rsx! {
        div { class: "pf-c-card__body", element }
    })
}

#[allow(non_snake_case)]
#[inline_props]
pub fn PfCardFooter<'a>(cx: Scope<'a>, element: Element<'a>) -> Element {
    cx.render(rsx! {
        div { class: "pf-c-card__footer", element }
    })
}
