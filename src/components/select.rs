use dioxus::prelude::*;

pub SelectProps<'a> {
    #[props(default)]
    color: Color,
    #[props(default)]
    size: Size,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    value: &'a str,
    #[props(default)]
    onchange: EventHandler<'a, String>,
}

pub fn Select(cx: Scope<'a, SelectProps<'a>>) -> Element {
    let color = cx.props.color.text_color();
    let font_size = cx.props.size.get_font_size();
    let disabled = cx.props.disabled;
    let value = cx.props.value;
    let onchange = cx.props.onchange;
    cx.render(rsx!(
        select {
            "type": "text",
            value: "{value}",
            color: "{color}",
            font_size: "{font_size}",
            disabled: "{disabled}",
            onchange: move |evt| {
                onchange.call(evt.target.value);
            }
        }
    ))
}