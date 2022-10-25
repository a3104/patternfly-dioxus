use std::time::Duration;

use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn PfToast<'a>(cx: Scope<'a>, timeout: Option<Duration>, children: Element<'a>) -> Element {
    let is_hidden = use_state(&cx, || false);
    if let Some(duration) = timeout {
        let duration = duration.clone();
        let is_hidden = is_hidden.clone();
        use_effect(&cx, (), |()| async move {
            // timer by gloo
            let _ = gloo::timers::future::sleep(duration).await;
            is_hidden.set(true);
        });
    }
    if !is_hidden.get() {
        cx.render(rsx! {
            span {
                hidden: "{is_hidden}",
                children,
            }
        })
    } else {
        cx.render(rsx!{ ""})
    }
}
