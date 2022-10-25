#[allow(non_snake_case, dead_code)]
mod accordion;
mod alert;
mod badge;
mod datepicker;
mod spiner;
mod tabs;
mod tooltip;
mod modal;
mod toast;
mod chip;
use std::time::Duration;

pub use accordion::*;
pub use alert::*;
pub use badge::*;
pub use datepicker::*;
pub use dioxus::prelude::*;
pub use spiner::*;
pub use tabs::*;
pub use tooltip::*;
pub use modal::*;
pub use toast::*;
pub use chip::*;


fn main() {
    dioxus::web::launch(App);
}
fn App(cx: Scope) -> Element {
    let date = use_state(&cx, || "2020-03-05".to_string());
    let modal_is_open = use_state(&cx, || false);
    let smallmodal_is_open = use_state(&cx, || false);
    let toaster_timer =  Some(Duration::from_secs(5));
    let chips = vec!["chip1".to_string(), "chip2".to_string(),"chip3".to_string()];
    let chip_states = use_state(&cx, || chips);
    let chips = chip_states.get().clone();
    let chips_str:String = chips.iter().map(|x| x.clone()).collect::<Vec<_>>().join(", ");

    cx.render(rsx! {
        div{
            "aaa",
            PfBadge {
                read: true,
                "1"
            }
            PfAlert {
                variation: Variation::Info,
                "bbb"
            }
            PfAlert {
                variation: Variation::Danger,
                title: "title",
                "body"
            }
            PfAccordion {
                title: "title",is_open:true,
                "body"
            }

            PfTooltip {
                content: "tooltip", orientation: Orientation::Right,
                "ccc"
            }
            PfDatePicker {
               date: date,
            }

            div { "{date}"}
            PfLargeSpinner {}

            PfMiddleSpinner {}
            PfSmallSpinner {}

            PfTabs {
                PfTab {
                    title: "tab1",
                    "tab1-content"
                },
                PfTab {
                    title: "tab2",
                    span {style:"color:red;", "tab2-content"}
                }
            }
            
        }
        PfModal {
            title: "modal", is_close: modal_is_open,
            "modal-content"
        }
        span {hidden: "true",
            PfSmallModal{
                title: "smallmodal", is_close: smallmodal_is_open,
                PfAlert{
                    variation: Variation::Danger,
                    title: "title",
                    "smallmodal-content"
                }
            }
    
        }

        PfToast {
            timeout: Duration::from_secs(5),
            PfAlert{
                variation: Variation::Danger,
                title: "title",
                "this alert will be closed after 5 seconds"
            }
        }

        PfChip{
            "chip"
        }
        PfChipGroup{
            chips: chip_states,
        }

        "{chips_str}"

    })
}
