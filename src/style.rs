#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TableStyle {
    #[default]
    Classic,
    Modern,
    Minimal,
    Compact,
    Markdown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderChars {
    pub vertical: &'static str,
    pub horizontal: &'static str,
    pub top_left: &'static str,
    pub top_right: &'static str,
    pub bottom_left: &'static str,
    pub bottom_right: &'static str,
    pub top_cross: &'static str,
    pub left_cross: &'static str,
    pub right_cross: &'static str,
    pub bottom_cross: &'static str,
    pub cross: &'static str,
}

impl TableStyle {
    #[must_use]
    pub fn border_chars(self) -> BorderChars {
        match self {
            TableStyle::Classic => BorderChars {
                vertical: "|",
                horizontal: "-",
                top_left: "+",
                top_right: "+",
                bottom_left: "+",
                bottom_right: "+",
                top_cross: "+",
                left_cross: "+",
                right_cross: "+",
                bottom_cross: "+",
                cross: "+",
            },
            TableStyle::Modern => BorderChars {
                vertical: "│",
                horizontal: "─",
                top_left: "┌",
                top_right: "┐",
                bottom_left: "└",
                bottom_right: "┘",
                top_cross: "┬",
                left_cross: "├",
                right_cross: "┤",
                bottom_cross: "┴",
                cross: "┼",
            },
            TableStyle::Minimal => BorderChars {
                vertical: " ",
                horizontal: "─",
                top_left: " ",
                top_right: " ",
                bottom_left: " ",
                bottom_right: " ",
                top_cross: " ",
                left_cross: "─",
                right_cross: "─",
                bottom_cross: " ",
                cross: "─",
            },
            TableStyle::Compact => BorderChars {
                vertical: "│",
                horizontal: "─",
                top_left: " ",
                top_right: " ",
                bottom_left: " ",
                bottom_right: " ",
                top_cross: " ",
                left_cross: "─",
                right_cross: "─",
                bottom_cross: " ",
                cross: "┼",
            },
            TableStyle::Markdown => BorderChars {
                vertical: "|",
                horizontal: "-",
                top_left: "|",
                top_right: "|",
                bottom_left: "|",
                bottom_right: "|",
                top_cross: "|",
                left_cross: "|",
                right_cross: "|",
                bottom_cross: "|",
                cross: "|",
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_equality() {
        let cases = [
            (TableStyle::Classic, TableStyle::Classic, true),
            (TableStyle::Modern, TableStyle::Modern, true),
            (TableStyle::Minimal, TableStyle::Minimal, true),
            (TableStyle::Compact, TableStyle::Compact, true),
            (TableStyle::Markdown, TableStyle::Markdown, true),
            (TableStyle::Classic, TableStyle::Modern, false),
            (TableStyle::Modern, TableStyle::Minimal, false),
            (TableStyle::Minimal, TableStyle::Compact, false),
            (TableStyle::Compact, TableStyle::Markdown, false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(a == b, expected);
        }
    }

    #[test]
    fn copy_trait() {
        let style = TableStyle::Modern;
        let copied = style;
        assert_eq!(style, copied);
    }

    #[test]
    fn border_chars_classic() {
        let chars = TableStyle::Classic.border_chars();
        assert_eq!(chars.vertical, "|");
        assert_eq!(chars.horizontal, "-");
        assert_eq!(chars.top_left, "+");
        assert_eq!(chars.cross, "+");
    }

    #[test]
    fn border_chars_modern() {
        let chars = TableStyle::Modern.border_chars();
        assert_eq!(chars.vertical, "│");
        assert_eq!(chars.horizontal, "─");
        assert_eq!(chars.top_left, "┌");
        assert_eq!(chars.cross, "┼");
    }

    #[test]
    fn border_chars_minimal() {
        let chars = TableStyle::Minimal.border_chars();
        assert_eq!(chars.vertical, " ");
        assert_eq!(chars.horizontal, "─");
        assert_eq!(chars.cross, "─");
        assert_eq!(chars.left_cross, "─");
        assert_eq!(chars.right_cross, "─");
    }

    #[test]
    fn border_chars_compact() {
        let chars = TableStyle::Compact.border_chars();
        assert_eq!(chars.vertical, "│");
        assert_eq!(chars.horizontal, "─");
        assert_eq!(chars.cross, "┼");
        assert_eq!(chars.left_cross, "─");
        assert_eq!(chars.right_cross, "─");
    }

    #[test]
    fn border_chars_markdown() {
        let chars = TableStyle::Markdown.border_chars();
        assert_eq!(chars.vertical, "|");
        assert_eq!(chars.horizontal, "-");
        assert_eq!(chars.top_left, "|");
        assert_eq!(chars.cross, "|");
    }
}
