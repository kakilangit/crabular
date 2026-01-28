/// Vertical alignment for multi-line cells
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VerticalAlignment {
    /// Align content to the top of the cell (default)
    #[default]
    Top,
    /// Align content to the middle of the cell
    Middle,
    /// Align content to the bottom of the cell
    Bottom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants() {
        let cases = [
            (VerticalAlignment::Top, VerticalAlignment::Top, true),
            (VerticalAlignment::Middle, VerticalAlignment::Middle, true),
            (VerticalAlignment::Bottom, VerticalAlignment::Bottom, true),
            (VerticalAlignment::Top, VerticalAlignment::Middle, false),
            (VerticalAlignment::Middle, VerticalAlignment::Bottom, false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(a == b, expected);
        }
    }

    #[test]
    fn default_is_top() {
        assert_eq!(VerticalAlignment::default(), VerticalAlignment::Top);
    }

    #[test]
    fn copy_trait() {
        let alignment = VerticalAlignment::Middle;
        let copied = alignment;
        assert_eq!(alignment, copied);
    }

    #[test]
    fn debug_trait() {
        assert_eq!(format!("{:?}", VerticalAlignment::Top), "Top");
        assert_eq!(format!("{:?}", VerticalAlignment::Middle), "Middle");
        assert_eq!(format!("{:?}", VerticalAlignment::Bottom), "Bottom");
    }
}
