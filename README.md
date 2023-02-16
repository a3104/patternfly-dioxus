# Patternfly components for Dioxus

[Dioxus](https://github.com/DioxusLabs/dioxus) Component for [Patternfly](https://www.patternfly.org/v4/).

## setup


```
npm i @patternfly/patternfly --save
```

append to index.html

```
    <link data-trunk rel="scss" href="node_modules/@patternfly/patternfly/patternfly.scss">
    <link data-trunk rel="scss" href="node_modules/@patternfly/patternfly/patternfly-addons.scss">
    <link data-trunk rel="copy-dir" href="node_modules/@patternfly/patternfly/assets">

```

## incompatible change

### v0.2.1

PfTab component: This component was using GlobalState, but we changed it to pass UseState to both parent and child.



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
* card
* Modal
* Chip
* Dropdown
* Toast
* DatePicker
* Tabs
* Label, LabelGroup


## working ...

* Navi
* Clipboard




for examples...

```
fn App(cx: Scope) -> Element {
    let date = use_state(&cx, || "2020-03-05".to_string());
    let modal_is_open = use_state(&cx, || false);
    let smallmodal_is_open = use_state(&cx, || false);
    let chips = vec!["chip1".to_string(), "chip2".to_string(),"chip3".to_string()];
    let chip_states = use_state(&cx, || chips);
    let chips = chip_states.get().clone();
    let chips_str:String = chips.iter().map(|x| x.clone()).collect::<Vec<_>>().join(", ");
    let str_state = use_state(&cx, || "".to_string());
    let vec_state: &UseState<Vec<String>> = use_state(&cx, || vec!["aaa".to_string(),"bbb".to_string()]);

    let tab_selected_contents = use_state(&cx, || "".to_string());

    cx.render(rsx! {
        div{
            "aaa",
            PfBadge {
                read: true,
                "1"
            }
            PfBadge {
                read: false,
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

            // Set the same state for the selected_contents attribute.
            // The content information of the tab selected by this UseState<String> is synchronized with the upper element.
            PfTabs {
                selected_contents: tab_selected_contents.clone(),
                PfTab {
                    selected_contents: tab_selected_contents.clone(),
                    title: "tab1",
                    "tab1-content"
                },
                PfTab {
                    selected_contents: tab_selected_contents.clone(),
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
            PfSmallModal{
                title: "toast", is_close: smallmodal_is_open,
                PfAlert{
                    variation: Variation::Danger,
                    title: "title",
                    "this modal will be close after 5 seconds"
                }
            }
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

        br{}
        PfDropDown{
            list: chips.clone(),selected: str_state.clone(),
        }

        PfDropDownRaw{
            selected: str_state.clone(),
            PfDropDownItem{
                item_str: "item1".to_string(),
                selected: str_state.clone(),

                div {
                    "item1"
                    i {class: "fas fa-angle-right", aria_hidden: "false" }
                }
            },
            PfDropDownItem{
                item_str: "item2".to_string(),
                selected: str_state.clone(),

                div {
                    "item2"
                    i {class: "fas fa-angle-right", aria_hidden: "false" }
                }
            },

               div { class: "pf-l-bullseye",
        
            PfCard {
                PfCardBody{
                    div {style:"color: red; width: 220px;text-align: center;","aaa",}
                }
            }
            PfCard {
                PfCardBody{
                    div {style:"color: red;","bbb",}
                }
            }
            
        }
        PfLabel{
            "label"
        }
        PfLabel {
            icon_name: "fa-info-circle",
            append_class: "pf-m-blue",
            "blue label"
        }
        PfLabelGroup{
            selected: vec_state,
        }

    })

        }
        div { class: "pf-l-bullseye",

            PfCard {
                PfCardBody{
                    div {style:"color: red; width: 220px;text-align: center;","aaa",}
                }
            }
            PfCard {
                PfCardBody{
                    div {style:"color: red;","bbb",}
                }
            }

        }
        PfLabel{
            "label"
        }
        PfLabel {
            icon_name: "fa-info-circle",
            append_class: "pf-m-blue",
            "blue label"
        }
        PfLabelGroup{
            selected: vec_state,
        }

    })
}
```
