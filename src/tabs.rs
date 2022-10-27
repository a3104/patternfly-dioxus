
use dioxus::{prelude::*};
use dioxus::fermi::prelude::*;


#[allow(non_snake_case)]
#[inline_props]
pub fn PfTabs<'a>(cx: Scope<'a>, selected_contents: UseState<String>, children: Element<'a>) -> Element<'a> {
    cx.render(rsx! {
        div { class: "pf-c-tabs",
            button { class: "pf-c-tabs__scroll-button", r#type: "button", disabled: "false", aria_hidden: "false", aria_label: "Scroll left",
                i { class: "fas fa-angle-left", aria_hidden: "true" }
            }
            ul { class: "pf-c-tabs__list",
                children
            }

        } 
        section {
            class: "pf-c-tab-content",
            role: "tabpanel",
            hidden: "false",
            span {
                dangerous_inner_html:"{selected_contents}",
            }

            
        }        
    }) 
}

#[allow(non_snake_case)]
#[inline_props]
pub fn PfTab<'a>(cx: Scope<'a>, title: &'a str,selected_contents: UseState<String>, children: Element<'a>) -> Element {

    //I didn't know how to use children to `use_atom_state`, So I converted the element to a string
    let children_string = dioxus::ssr::render_lazy(rsx!{span{children}});
    
    
    cx.render(rsx! {
        li { class: "pf-c-tabs__item",
            button { r#type: "button", class: "pf-c-tabs__link", onclick: move |_| {selected_contents.set(children_string.clone());}  ,  "{title}"
                span { class: "pf-c-tabs__item-text"}
            }
        }
    })
}

