use dioxus::{events::MouseEvent, prelude::*};

use crate::size::Size;
use crate::color::Color;
#[derive(Props)]
pub struct CheckboxProps<'a> {
	#[props(default)]
	checked: bool,

	#[props(default)]
	disabled: bool,

	#[props(default)]
	color: Color,

	#[props(default)]
	bg_color: Color,

	#[props(default)]
	size: Size,
	
	label: &'a str,

	onclick: EventHandler<'a, MouseEvent>,
}

pub fn Checkbox<'a>(cx: Scope<'a, CheckboxProps<'a>>) -> Element{
	let color = cx.props.color.text_color();
	cx.render(rsx!{
		div {
			display: "inline-flex",
			input {
				display: "none",
				checked: "{cx.props.checked}",
			}
			label {
				color: "{color}",
				"{cx.props.label}"
			}
		}
	})
}