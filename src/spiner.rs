use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn PfLargeSpinner(cx: Scope) -> Element {

    
    cx.render(rsx! {
        svg { class: "pf-c-spinner", role: "progressbar", view_box: "0 0 100 100",
            circle { class: "pf-c-spinner__path", cx: "50", cy: "50", r: "45", fill: "none" }
        }
    
    })
}

#[allow(non_snake_case)]
pub fn PfSmallSpinner(cx: Scope) -> Element {

    
    cx.render(rsx! {
        svg { class: "pf-c-spinner pf-m-sm", role: "progressbar", view_box: "0 0 100 100",
            circle { class: "pf-c-spinner__path", cx: "50", cy: "50", r: "45", fill: "none" }
        }
    
    })
}

#[allow(non_snake_case)]
pub fn PfMiddleSpinner(cx: Scope) -> Element {
    cx.render(rsx! {
        svg { class: "pf-c-spinner pf-m-md", role: "progressbar", view_box: "0 0 100 100",
            circle { class: "pf-c-spinner__path", cx: "50", cy: "50", r: "45", fill: "none" }
        }
    
    })
}
