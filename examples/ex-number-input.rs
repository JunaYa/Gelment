use dioxus::prelude::*;
use gelement::{prelude::*, size::Size};

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cxt: Scope) -> Element {
    cxt.render(rsx!(
        div {
            div {
                NumberInput {
                    value: 0,
                    size: Size::TINY,
                    label: "Number Input",
                },
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::SMALL,
                    label: "Number Input",
                },
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::MEDIUM,
                    label: "Number Input",
                },
            }   
            div {
                NumberInput {
                    value: 0,
                    size: Size::LARGE,
                    label: "Number Input",
                }
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::XLARGE,
                    label: "Number Input",
                }
            }
            div {
                NumberInput {
                    value: 0,
                    size: Size::XXLARGE,
                    label: "Number Input",
                }
            }         
        }
    ))
}