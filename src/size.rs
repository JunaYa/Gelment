#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
	TINY,
	SMALL,
	MEDIUM,
	LARGE,
	XLARGE,
	XXLARGE,
	Default,
}

impl Size {
	pub fn to_string(self) -> String {
		match self {
			Size::TINY => "-tiny".into(),
			Size::SMALL => "-small".into(),
			Size::MEDIUM => "-medium".into(),
			Size::LARGE => "-large".into(),
			Size::XLARGE => "-xlarge".into(),
			Size::XXLARGE => "-xxlarge".into(),
			Size::Default => "".into(),
		}
	}

	pub fn get_padding(self) -> Box<str> {
		match self {
			Size::TINY => "4px 6px".into(),
			Size::SMALL => "8px 12px".into(),
			Size::MEDIUM => "10px 24px".into(),
			Size::LARGE => "12px 28px".into(),
			Size::XLARGE => "14px 32px".into(),
			Size::XXLARGE => "20px 40px".into(),
			Size::Default => "none".into(),
		}
	}
}

impl Default for Size {
	fn default() -> Self {
		Size::Default
	}
}