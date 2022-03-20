use dioxus::prelude::*;

#[derive(Props)]
pub struct ListProps {
    data: Vec<String>,
    children: Element,
}

pub fn List(cx: Scope) -> Element {
    cx.render(rsx!(
        cx.props.children,
    ))
}