#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
	Transparent,
	Black,
	White,
	Slate,
	Gray,
	Zinc,
	Neutral,
	Stone,
	Red,
	Orange,
	Amber,
	Yellow,
	Lime,
	Green,
	Emerald,
	Teal,
	Cyan,
	Sky,
	Blue,
	Indigo,
	Violet,
	Purple,
	Fuchsia,
	Pink,
	Rose,
	Default,
}

impl Default for Color {
	fn default() -> Self {
		Color::Default
	}
}

impl Color {
	pub fn text_color(self) -> Box<str> {
		match self {
			Color::Transparent => "rgb(0, 0, 0, 0)".into(),
			Color::Black => "rgb(0, 0, 0)".into(),
			Color::White => "rgb(255, 255, 255)".into(),
			Color::Slate => "rgb(100, 116, 139)".into(),
			Color::Gray => "rgb(107, 114, 128)".into(),
			Color::Zinc => "rgb(113, 113, 122)".into(),
			Color::Neutral => "rgb(115, 115, 115)".into(),
			Color::Stone => "rgb(120, 113, 108)".into(),
			Color::Red => "rgb(239, 68, 68)".into(),
			Color::Orange => "rgb(249, 115, 22)".into(),
			Color::Amber => "rgb(245, 158, 11)".into(),
			Color::Yellow => "rgb(234, 179, 8)".into(),
			Color::Lime => "rgb(132, 204, 22)".into(),
			Color::Green => "rgb(34, 197, 94)".into(),
			Color::Emerald => "rgb(16, 185, 129)".into(),
			Color::Teal => "rgb(20, 184, 166)".into(),
			Color::Cyan => "rgb(6, 182, 212)".into(),
			Color::Sky => "rgb(14, 165, 233)".into(),
			Color::Blue => "rgb(59, 130, 246)".into(),
			Color::Indigo => "rgb(99, 102, 241)".into(),
			Color::Violet => "rgb(139, 92, 246)".into(),
			Color::Purple => "rgb(168, 85, 247)".into(),
			Color::Fuchsia => "rgb(217, 70, 239)".into(),
			Color::Pink => "rgb(236, 72, 153)".into(),
			Color::Rose => "rgb(244, 63, 94)".into(),
			Color::Default => "".into(),
		}
	}

	pub fn bg_color(self) -> Box<str> {
		match self {
			Color::Transparent => "rgb(0, 0, 0, 0)".into(),
			Color::Black => "rgb(0, 0, 0)".into(),
			Color::White => "rgb(255, 255, 255)".into(),
			Color::Slate => "rgb(100, 116, 139)".into(),
			Color::Gray => "rgb(107, 114, 128)".into(),
			Color::Zinc => "rgb(113, 113, 122)".into(),
			Color::Neutral => "rgb(115, 115, 115)".into(),
			Color::Stone => "rgb(120, 113, 108)".into(),
			Color::Red => "rgb(239, 68, 68)".into(),
			Color::Orange => "rgb(249, 115, 22)".into(),
			Color::Amber => "rgb(245, 158, 11)".into(),
			Color::Yellow => "rgb(234, 179, 8)".into(),
			Color::Lime => "rgb(132, 204, 22)".into(),
			Color::Green => "rgb(34, 197, 94)".into(),
			Color::Emerald => "rgb(16, 185, 129)".into(),
			Color::Teal => "rgb(20, 184, 166)".into(),
			Color::Cyan => "rgb(6, 182, 212)".into(),
			Color::Sky => "rgb(14, 165, 233)".into(),
			Color::Blue => "rgb(59, 130, 246)".into(),
			Color::Indigo => "rgb(99, 102, 241)".into(),
			Color::Violet => "rgb(139, 92, 246)".into(),
			Color::Purple => "rgb(168, 85, 247)".into(),
			Color::Fuchsia => "rgb(217, 70, 239)".into(),
			Color::Pink => "rgb(236, 72, 153)".into(),
			Color::Rose => "rgb(244, 63, 94)".into(),
			Color::Default => "".into(),
		}
	}
}