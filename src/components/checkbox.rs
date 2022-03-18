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
	let cursor = if cx.props.disabled == true { "not-allowed" } else { "pointer" };
	let mut checked = cx.use_hook(|_| false);

	cx.render(rsx!{
		div {
			display: "inline-flex",
			justify_content: "center",
			align_items: "center",
			cursor: "{cursor}",
			onclick: move |evt| {
				println!("before {}", checked);
				cx.props.onclick.call(evt);
				*checked = match checked {
					true => false,
					false => true,
				};
				println!("after {}", checked);
			},
			div {
				width: ".8rem",
				height: ".8rem",
				border: ".15rem solid {color}",
				border_radius: ".3rem",
				background_color: "#fff",
				margin_right: ".4rem",
				box_shadow: "0.15rem 0.15rem 0.3rem #c8d0e7, -0.1rem -0.1rem 0.25rem #FFFFFF",
			}
			input {
				display: "none",
				r#type: "checkbox",
				checked: "{cx.props.checked}",
			}
			label {
				color: "{color}",
				"{cx.props.label}"
			}
		}
	})
}