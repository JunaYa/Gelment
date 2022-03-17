use dioxus::prelude::*;
use gelement::{prelude::*, size::Size};

fn main() {
    // Add this line:
    dioxus::desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            Button {
                size: Size::TINY,
                text: "GButton-tiny",
                onclick: move |_| {
                    println!("GButton TINY");
                }
            }
			Button {
                size: Size::MEDIUM,
                text: "GButton-medium",
                onclick: move |_| {
                    println!("GButton MEDIUM");
                }
            }
			Button {
                size: Size::LARGE,
                text: "GButton-large",
                onclick: move |_| {
                    println!("GButton LARGE");
                }
            }
			Button {
                size: Size::XLARGE,
                text: "GButton-xlarge",
                onclick: move |_| {
                    println!("GButton XLARGE");
                }
            }
			Button {
                size: Size::XXLARGE,
                text: "GButton-xxlarge",
                onclick: move |_| {
                    println!("GButton XXLARGE");
                }
            }
		}
		div {
            Button {
				disabled: true,
                size: Size::TINY,
                text: "GButton-tiny",
                onclick: move |_| {
                    println!("GButton TINY");
                }
            }
			Button {
				disabled: true,
                size: Size::MEDIUM,
                text: "GButton-medium",
                onclick: move |_| {
                    println!("GButton MEDIUM");
                }
            }
			Button {
				disabled: true,
                size: Size::LARGE,
                text: "GButton-large",
                onclick: move |_| {
                    println!("GButton LARGE");
                }
            }
			Button {
				disabled: true,
                size: Size::XLARGE,
                text: "GButton-xlarge",
                onclick: move |_| {
                    println!("GButton XLARGE");
                }
            }
			Button {
				disabled: true,
                size: Size::XXLARGE,
                text: "GButton-xxlarge",
                onclick: move |_| {
                    println!("GButton XXLARGE");
                }
            }
		}
    ))
}