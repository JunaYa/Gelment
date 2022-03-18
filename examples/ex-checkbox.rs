use dioxus::{prelude::*, events::MouseEvent};
use gelement::{prelude::*, size::Size, color::Color};

fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let v_color: Vec<Color> = vec![
        Color::Transparent,
        Color::Black,
        Color::White,
        Color::Slate,
        Color::Gray,
        Color::Zinc,
        Color::Neutral,
        Color::Stone,
        Color::Red,
        Color::Orange,
        Color::Amber,
        Color::Yellow,
        Color::Lime,
        Color::Green,
        Color::Emerald,
        Color::Teal,
        Color::Cyan,
        Color::Sky,
        Color::Blue,
        Color::Indigo,
        Color::Violet,
        Color::Purple,
        Color::Fuchsia,
        Color::Pink,
        Color::Rose,
        Color::Default,
    ];
    let mut list_checkgroup: Vec<CheckGroupsItem> = vec![];
    for i in 0..v_color.len() {
        let item = CheckGroupsItem {
            id: i,
            color: v_color[i],
            checked: i % 2 == 0,
        };
        list_checkgroup.push(item);
    }
    cx.render(rsx! (
        list_checkgroup.iter().map(|item| {
            rsx!(
                GCheckboxColors {
                    id: item.id,
                    color: item.color,
                    onclick: move|evt| {
                        println!("click {:?}", evt);
                    },
                    checked: item.checked,
                }
            )
        })
    ))
}

#[derive(Props, PartialEq, Clone, Debug)]
struct CheckGroupsItem {
    id: usize,
    color: Color,
    checked: bool,
}

#[derive(Props)]
struct BgColorProps<'a> {
    id: usize,
    color: Color,
    checked: bool,
    onclick: EventHandler<'a, MouseEvent>,
}
#[warn(non_snake_case)]
fn GCheckboxColors<'a>(cx: Scope<'a, BgColorProps<'a>>) -> Element {
    cx.render(rsx!(
        div {
            Checkbox {
                disabled: false,
                checked: cx.props.checked,
                size: Size::TINY,
                color: cx.props.color,
                bg_color: cx.props.color,
                label: "GCheckbox-tiny",
                onclick: move |evt| {
                    println!("GCheckbox TINY");
                    cx.props.onclick.call(evt);
                }
            }
            Checkbox {
                disabled: false,
                checked: cx.props.checked,
                size: Size::MEDIUM,
                color: cx.props.color,
                bg_color: cx.props.color,
                label: "GCheckbox-medium",
                onclick: move |evt| {
                    println!("GCheckbox MEDIUM");
                    cx.props.onclick.call(evt);
                }
            }
            Checkbox {
                disabled: false,
                checked: cx.props.checked,
                size: Size::LARGE,
                color: cx.props.color,
                bg_color: cx.props.color,
                label: "GCheckbox-large",
                onclick: move |evt| {
                    println!("GCheckbox LARGE");
                    cx.props.onclick.call(evt);
                }
            }
            Checkbox {
                disabled: false,
                checked: cx.props.checked,
                size: Size::XLARGE,
                color: cx.props.color,
                bg_color: cx.props.color,
                label: "GCheckbox-xlarge",
                onclick: move |evt| {
                    println!("GCheckbox XLARGE");
                    cx.props.onclick.call(evt);
                }
            }
            Checkbox {
                disabled: true,
                checked: cx.props.checked,
                size: Size::XXLARGE,
                label: "GCheckbox-xxlarge",
                color: cx.props.color,
                bg_color: cx.props.color,
                onclick: move |evt| {
                    println!("GCheckbox XXLARGE");
                    cx.props.onclick.call(evt);
                }
            }
        }
    ))
}