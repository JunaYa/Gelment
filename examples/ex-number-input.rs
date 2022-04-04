use dioxus::prelude::*;
use gelement::{prelude::*};

fn main() {
    dioxus::desktop::launch(app);
}

fn app(cxt: Scope) -> Element {
    cxt.render(rsx!(
        div {
            NumberInput {
                value: 0,
                label: "Number Input",
            },
        }
    ))
}