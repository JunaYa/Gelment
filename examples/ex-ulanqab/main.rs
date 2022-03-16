use dioxus::prelude::*;
fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            h1 {
                color: "red",
                font_size: "80px",
                "h1"
            }
         }
    ))
}