use dioxus::prelude::*;

#[allow(non_snake_case,dead_code)]
#[inline_props]
pub fn PfLabel<'a>(cx: Scope<'a>, icon_name: Option<&'a str>, append_class: Option<&'a str>, children: Element<'a>) -> Element {
    let append_class = format!("pf-c-label {}", append_class.unwrap_or(""));
    if icon_name.is_some() {
        let icon_name = icon_name.unwrap();
        cx.render(rsx! {
            span { class: "{append_class}",
                i { class: "fas fa-fw {icon_name}", aria_hidden: "true", } 
                children
            }
        })
    } else {
        cx.render(rsx! {
            span { class: "{append_class}", children }
        })
    }
}

#[allow(non_snake_case,dead_code)]
#[inline_props]
pub fn PfLabelGroup<'a> (cx: Scope, selected: &'a UseState<Vec<String>>) -> Element {
    let is_close = use_state(&cx, || false);
    let labels = selected.get().clone();
    cx.render(rsx! {
        div { class: "pf-c-label-group",
            labels.into_iter().map(|label| rsx!{
                div {
                    onclick: move |_| {is_close.set(!is_close.get());},
                    PfLabel {  "{label}" 
                        i {
                            class: "fas fa-times-circle pf-c-label__close",
                            aria_hidden: "true",
                            hidden: "{is_close}",
                            onclick: move |_| {selected.set(selected.get().iter().filter(|x| **x != label).map(|x| x.to_string()).collect())}
                        }
                    }
                }
            })
        }
    })
}
