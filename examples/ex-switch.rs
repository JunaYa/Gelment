use dioxus::prelude::*;
use gelement::{
    prelude::*,
    size::Size,
    color::Color,
    constants::getColors,
};

fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let v_color = getColors();
    cx.render(rsx!(
        v_color.iter().map(move |color| {
            rsx!(
                div {
                    Switch {
                        color: *color,
                        size: Size::TINY,
                        checked: false,
                    }
                    Switch {
                        color: *color,
                        size: Size::SMALL,
                        checked: true,
                    }
                    Switch {
                        color: *color,
                        size: Size::MEDIUM,
                        checked: true,
                    }
                    Switch {
                        color: *color,
                        size: Size::LARGE,
                        checked: true,
                    }
                    Switch {
                        color: *color,
                        size: Size::XLARGE,
                    }
                    Switch {
                        color: *color,
                        size: Size::XXLARGE,
                        checked: true,
                    }
                }
            )
        })
    ))
}