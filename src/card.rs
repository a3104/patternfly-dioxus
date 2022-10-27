use dioxus::prelude::*;

#[allow(non_snake_case,dead_code)]
#[inline_props]
pub fn PfCard<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div { class: "pf-c-card", children }
    })
}

#[allow(non_snake_case,dead_code)]
#[inline_props]
pub fn PfCardTitle<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div { class: "pf-c-card__title", children }
    })
}

#[allow(non_snake_case,dead_code)]
#[inline_props]
pub fn PfCardBody<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div { class: "pf-c-card__body", children }
    })
}

#[allow(non_snake_case)]
#[inline_props]
pub fn PfCardFooter<'a>(cx: Scope<'a>, children: Element<'a>) -> Element {
    cx.render(rsx! {
        div { class: "pf-c-card__footer", children }
    })
}
