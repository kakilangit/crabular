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

impl core::str::FromStr for VerticalAlignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "top" | "t" => Ok(VerticalAlignment::Top),
            "middle" | "m" | "center" => Ok(VerticalAlignment::Middle),
            "bottom" | "b" => Ok(VerticalAlignment::Bottom),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::VerticalAlignment;

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

    #[test]
    fn from_str() {
        assert_eq!("top".parse(), Ok(VerticalAlignment::Top));
        assert_eq!("t".parse(), Ok(VerticalAlignment::Top));
        assert_eq!("middle".parse(), Ok(VerticalAlignment::Middle));
        assert_eq!("m".parse(), Ok(VerticalAlignment::Middle));
        assert_eq!("center".parse(), Ok(VerticalAlignment::Middle));
        assert_eq!("bottom".parse(), Ok(VerticalAlignment::Bottom));
        assert_eq!("b".parse(), Ok(VerticalAlignment::Bottom));
        assert_eq!("invalid".parse::<VerticalAlignment>(), Err(()));
    }
}
