use dioxus::{events::MouseEvent, prelude::*};

use crate::size::Size;
use crate::color::Color;
#[derive(Props)]
pub struct CheckboxProps<'a> {
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
	let c = Color::Gray;
	let default_color = c.text_color();
	let cursor = if cx.props.disabled { "not-allowed" } else { "pointer" };
	let color = if cx.props.checked {cx.props.color.text_color()} else {default_color};
	let size = cx.props.size.get_checkbox();
	let font_size = cx.props.size.get_font_size();
	cx.render(rsx!{
		div {
			display: "inline-flex",
			justify_content: "center",
			align_items: "center",
			cursor: "{cursor}",
			onclick: move |evt| {
				cx.props.onclick.call(evt);
			},
			div {
				width: "{size}",
				height: "{size}",
				border: ".15rem solid {color}",
				border_radius: ".3rem",
				background_color: "#fff",
				margin_right: ".4rem",
				// box_shadow: "0.15rem 0.15rem 0.3rem #c8d0e7, -0.1rem -0.1rem 0.25rem #FFFFFF",
			}
			input {
				display: "none",
				r#type: "checkbox",
				checked: "{cx.props.checked}",
			}
			label {
				font_size: "{font_size}",
				color: "{color}",
				"{cx.props.label}"
			}
		}
	})
}