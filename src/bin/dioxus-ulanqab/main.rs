use dioxus::prelude::*;
mod components;
fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            Button {
                text: String::from("MyButton")
            }
         }
    ))
}