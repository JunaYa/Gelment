use dioxus::prelude::*;
fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let enabled = true;
    let age = if enabled { 12 } else { 20 };
    // use_state
    let value = use_state(&cx, || String::from("hello world"));
    let name = use_state(&cx, || String::from("aya"));
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
            ElName {
                name: String::from("Junaya")
            }
            p { "p" }
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
            VoteButton { score: 32 }
         }
    ))
}

#[derive(PartialEq, Props)]
struct NameProps {
    name: String,
}

fn ElName(cx: Scope<NameProps>) -> Element {
    cx.render(rsx![
        div {
            "{cx.props.name}"
        }
    ])
}

#[derive(PartialEq, Props)]
struct VoteButtonProps {
    score: i32,
}

fn VoteButton(cx: Scope<VoteButtonProps>) -> Element {
    cx.render(rsx![
        div {
            div { "+" }
            div { "{cx.props.score}"}
            div { "-" }
        }
    ])
}