mod accordion;
mod alert;
#[allow(non_snake_case, dead_code)]
mod badge;
mod tooltip;
pub use tooltip::*;
pub use accordion::*;
pub use alert::*;
pub use badge::*;
pub use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(App);
}
fn App(cx: Scope) -> Element {
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
                description: "body",
                "bbb"
            }
            PfAccordion {
                title: "title",is_open:true,
                "body"
            }

            PfTooltip {
                content: "tooltip", orientation: Orientation::Right,
                "ccc"
            }

        }

    })
}
