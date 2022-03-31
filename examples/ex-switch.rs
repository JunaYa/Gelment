use dioxus::prelude::*;
use gelement::{
    prelude::*,
    size::Size,
    color::Color,
};

fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let v_color = vec![
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
    cx.render(rsx!(
        v_color.iter().map(move |color| {
            rsx!(
                div {
                    Switch {
                        color: *color,
                        size: Size::TINY,
                    }
                }
            )
        })
    ))
}