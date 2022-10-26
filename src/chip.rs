use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn PfChip<'a>(cx: Scope<'a>, use_close_button: Option<bool>, children: Element<'a>) -> Element {
    let use_close_button = use_close_button.unwrap_or(true);
    let is_close = use_state(&cx, || false);

    if *is_close.get() {
        None
    } else {
        cx.render(rsx! {

            span { class: "pf-c-chip",
                span { class: "pf-c-chip__text", children },
            
                use_close_button.then(|| rsx!{
                    button {
                        onclick: |_| { is_close.set(true); },
                        class: "pf-c-button pf-m-plain",
                        r#type: "button",
                        aria_labelledby: "remove_chip_one chip_one",
                        aria_label: "Remove",
                        i {
                            class: "fas fa-times",
                            aria_hidden: "true",
                        }
                    }
                })
        }
        })
    }
}

#[allow(non_snake_case)]
#[inline_props]
pub fn PfChipGroup<'a>(cx: Scope<'a>, chips: &'a UseState<Vec<String>>) -> Element {
    let chips = chips.clone();

    cx.render(rsx! {
        div {
            class: "pf-c-chip-group",
            div {
                class: "pf-c-chip-group__main",
                ul {
                    class: "pf-c-chip-group__list",
                    role: "list",
                    aria_label: "Chip group list",
                    
                    chips.get().iter().map(|chip| rsx!{
                        li {
                            class: "pf-c-chip-group__list-item",
                            span{
                                PfChip { use_close_button: false, "{chip}" 
                                    button {
                                        onclick: move |_|  {
                                            let chips = chips.clone();
                                            let chip_list: Vec<String> = chips.get().iter().filter(|x| *x != chip).map(|x| x.clone()).collect();
                                            chips.set(chip_list); 
                                        },
                                        class: "pf-c-button pf-m-plain",
                                        r#type: "button",
                                        aria_labelledby: "remove_chip_one chip_one",
                                        aria_label: "Remove",
                                        i {
                                            class: "fas fa-times",
                                            aria_hidden: "true",
                                        }
                                    }
                                }
                            }
                        }
                    })
                }
            }
        }
    })
}
