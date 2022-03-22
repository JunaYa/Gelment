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
            Size::Default => "-medium".into(),
        }
    }

    pub fn get_padding(self) -> Box<str> {
        match self {
            Size::TINY => ".25rem .375rem".into(),
            Size::SMALL => ".5rem .75rem".into(),
            Size::MEDIUM => ".625rem 1.5rem".into(),
            Size::LARGE => ".75rem 1.75rem".into(),
            Size::XLARGE => ".875rem 2rem".into(),
            Size::XXLARGE => "1.25rem 2.5rem".into(),
            Size::Default => ".625rem 1.5rem".into(),
        }
    }

    pub fn get_font_size(self) -> Box<str> {
        match self {
            Size::TINY => ".75rem".into(),
            Size::SMALL => ".875rem".into(),
            Size::MEDIUM => "1rem".into(),
            Size::LARGE => "1.125rem".into(),
            Size::XLARGE => "1.5rem".into(),
            Size::XXLARGE => "1.875rem".into(),
            Size::Default => "1rem".into(),
        }
    }

    pub fn get_border_radius(self) -> Box<str> {
        match self {
            Size::TINY => "1.25rem".into(),
            Size::SMALL => "1.875rem".into(),
            Size::MEDIUM => "2.25rem".into(),
            Size::LARGE => "2.625rem".into(),
            Size::XLARGE => "3.25rem".into(),
            Size::XXLARGE => "4.375rem".into(),
            Size::Default => "2.25rem".into(),
        }
    }
    pub fn get_radio(self) -> Box<str> {
        match self {
            Size::TINY => ".8rem".into(),
            Size::SMALL => ".875rem".into(),
            Size::MEDIUM => "1rem".into(),
            Size::LARGE => "1.25rem".into(),
            Size::XLARGE => "1.5rem".into(),
            Size::XXLARGE => "1.75rem".into(),
            Size::Default => "1rem".into(),
        }
    }
}

impl Default for Size {
    fn default() -> Self {
        Size::Default
    }
}
