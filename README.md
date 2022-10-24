# Patternfly components for Yew

[Dioxus](https://github.com/DioxusLabs/dioxus) Component for [Patternfly](https://www.patternfly.org/v4/).

## Currently available components.

* PfBadge 
* PfAlert
* PfAccordion
* PfTooltip
* PfDatePicker
* Spinner
    * PfLargeSpinner
    * PfMiddleSpinner
    * PfSmallSpinner
* PfTabs





for examples...

```
fn App(cx: Scope) -> Element {
    let date = use_state(&cx, || "2020-03-05".to_string());
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

    })
```
