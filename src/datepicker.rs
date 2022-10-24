use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn PfDatePicker<'a>(cx: Scope<'a>, date: &'a UseState<String>) -> Element {
    cx.render(rsx! {
        div{ class: "pf-c-date-picker",
            div { class: "pf-c-datepicker__input",
                div { class: "pf-c-input-group",
                    input { class: "pf-c-form-control", r#type: "date", value: "{date}", aria_label: "Date picker",onchange: move |e| { date.set(e.value.to_string()); }, }
                    
                }
            }
        }
    })
}

