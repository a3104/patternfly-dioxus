use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn PfBadge<'a>(cx: Scope<'a>, children: Element<'a>, read:Option<bool>) -> Element {
    let read = read.unwrap_or(false);
    let css = if read {
        "pf-c-badge pf-m-read"
    } else {
        "pf-c-badge pf-m-unread"
    };
    cx.render(rsx! {
        span{ class:"{css}", children}
        
    })
}
