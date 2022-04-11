use dioxus::prelude::*;
use gelement::{prelude::*, size::Size, color::Color, constants::getColors};


fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let v_color: Vec<Color> = getColors();
    cx.render(rsx!(
        v_color.iter().map(|color| {
            rsx!(
                div {
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::TINY,
                    }
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::SMALL,
                    }
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::MEDIUM,
                    }
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::LARGE,
                    }
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::XLARGE,
                    }
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::XXLARGE,
                    }
                }
            )
        })
    ))
}