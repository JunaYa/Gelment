use dioxus::prelude::*;


fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let name = "Aya";
    let enabled = true;
    let age = if enabled { 12 } else { 20 };
    // use_state
    let value = use_state(&cx, || String::from("hello world"));
    cx.render(rsx! (
        div {
            h1 {
                color: "red",
                font_size: "80px",
                "h1"
            }
            h2 {
                hidden: "true",
                "h2"
            }
            button {
                onclick: move |_|{},
                "button"
            }
            p { "p" }
            p { "p2 {name} {age}" }
            p {
                enabled.then(|| rsx!{
                    "This is the p"
                })
            }
            input {
                oninput: move |_| {},
                prevent_default: "oninput",

                onclick: move |_| {},
                prevent_default: "onclick",
                value: "{value}",
            }
            input {
                oninput: move |evt| value.set(evt.value.clone()),
                value: "{value}",
            }
         }
    ))
}
