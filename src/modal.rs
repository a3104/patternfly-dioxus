use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn PfSmallModal<'a>(cx: Scope<'a>, title: &'a str, children: Element<'a>, is_close: &'a UseState<bool>) -> Element {

    // <div
//   class="pf-c-modal-box pf-m-sm"
//   role="dialog"
//   aria-modal="true"
//   aria-labelledby="modal-sm-title"
//   aria-describedby="modal-sm-description"
// >
//   <button
//     class="pf-c-button pf-m-plain"
//     type="button"
//     aria-label="Close dialog"
//   >
//     <i class="fas fa-times" aria-hidden="true"></i>
//   </button>
//   <header class="pf-c-modal-box__header">
//     <h1 class="pf-c-modal-box__title" id="modal-sm-title">Modal title</h1>
//   </header>
//   <div class="pf-c-modal-box__body" id="modal-sm-description">
//     Static text describing modal purpose. Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
//     tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
//     quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
//     consequat.
//   </div>
//   <footer class="pf-c-modal-box__footer">Modal footer</footer>
// </div>
    cx.render(rsx!{
        div {
            class: "pf-c-backdrop",
            hidden: "{is_close}",
            div{ class: "pf-c-modal-box pf-m-sm",
                style: "top:50%;left:50%;transform:translate(-50%,-50%);",
                role: "dialog",
                aria_modal: "true",
                aria_labelledby: "modal-sm-title",
                aria_describedby: "modal-sm-description",
                button { class: "pf-c-button pf-m-plain", r#type: "button", aria_label: "Close dialog",
                    onclick: move |_| { is_close.set(true); },
                    i { class: "fas fa-times", aria_hidden: "true" }
                },
                header { class: "pf-c-modal-box__header",
                    h1 { class: "pf-c-modal-box__title", "{title}" }
                },
                div { class: "pf-c-modal-box__body", children }
            }
    }
    })
 
}



#[allow(non_snake_case)]
#[inline_props]
pub fn PfModal<'a>(cx: Scope<'a>, title: &'a str, children: Element<'a>, is_close: &'a UseState<bool>) -> Element {
    // let is_close = use_state(&cx, || false);

    cx.render(rsx!{
        div {
            class: "pf-c-backdrop",
            hidden: "{is_close}",
            div {
                class: "pf-c-modal-box",
                style: "top:50%;left:50%;transform:translate(-50%,-50%);",
                role: "dialog",
                aria_modal: "true",
                aria_labelledby: "modal-title",
                aria_describedby: "modal-description",
                button {
                    class: "pf-c-button pf-m-plain",
                    r#type: "button",
                    aria_label: "Close",
                    onclick: move |_| { is_close.set(true); },
                    i {
                        class: "fas fa-times",
                        aria_hidden: "true",
                    }
                },
                header {
                    class: "pf-c-modal-box__header",
                    h1 {
                        class: "pf-c-modal-box__title",
                        id: "modal-title",
                        "{title}"
                    }
                },
                div {
                    class: "pf-c-modal-box__body",
                    children 
                },
                
            }
        }
    
    })
}
