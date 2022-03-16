use dioxus::prelude::*;
mod components;
use crate::components::button::Button;
fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            Button {
                text: String::from("GButton")
            }
         }
    ))
}