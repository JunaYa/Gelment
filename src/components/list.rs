use dioxus::prelude::*;

#[derive(Props)]
pub struct ListProps<'a> {
    data: Vec<String>,
    children: Element<'a>,
}

pub fn List<'a>(cx: Scope<'a, ListProps<'a>>) -> Element {
    cx.render(rsx!(
        &cx.props.children,
    ))
}