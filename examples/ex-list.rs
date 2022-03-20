use dioxus::prelude::*;
fn main() {
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let mock_data = vec![String::from("data1"), String::from("data2")];

    cx.render(rsx!(
        mock_data.iter().map(|item| {
            rsx!(
                div {
                    "{item}"
                }
            )
        })
    ))
}