use dioxus::prelude::*;
use gelement::{prelude::*, size::Size, color::Color, constants::getColors};


fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let v_color: Vec<Color> = getColors();
    // let options = vec![("1", "Medium"), ("2", "Small"), ("3", "Large"), ("4", "XLarge"), ("5", "XXLarge")];
    cx.render(rsx!(
        v_color.iter().map(|color| {
            rsx!(
                div {
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::TINY,
                        options: vec![("1", "Medium"), ("2", "Small"), ("3", "Large"), ("4", "XLarge"), ("5", "XXLarge")],
                    }
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::SMALL,
                        options: vec![("1", "Medium"), ("2", "Small"), ("3", "Large"), ("4", "XLarge"), ("5", "XXLarge")],
                    }
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::MEDIUM,
                        options: vec![("1", "Medium"), ("2", "Small"), ("3", "Large"), ("4", "XLarge"), ("5", "XXLarge")],
                    }
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::LARGE,
                        options: vec![("1", "Medium"), ("2", "Small"), ("3", "Large"), ("4", "XLarge"), ("5", "XXLarge")],
                    }
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::XLARGE,
                        options: vec![("1", "Medium"), ("2", "Small"), ("3", "Large"), ("4", "XLarge"), ("5", "XXLarge")],
                    }
                    Select{
                        disabled: false,
                        color: *color,
                        size: Size::XXLARGE,
                        options: vec![("1", "Medium"), ("2", "Small"), ("3", "Large"), ("4", "XLarge"), ("5", "XXLarge")],
                    }
                }
            )
        })
    ))
}