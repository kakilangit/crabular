#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants() {
        let cases = [
            (Alignment::Left, Alignment::Left, true),
            (Alignment::Center, Alignment::Center, true),
            (Alignment::Right, Alignment::Right, true),
            (Alignment::Left, Alignment::Right, false),
            (Alignment::Left, Alignment::Center, false),
            (Alignment::Center, Alignment::Right, false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(a == b, expected);
        }
    }

    #[test]
    fn copy_trait() {
        let alignment = Alignment::Left;
        let copied = alignment;
        assert_eq!(alignment, copied);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn clone_trait() {
        let alignment = Alignment::Center;
        let cloned = alignment.clone();
        assert_eq!(alignment, cloned);
    }

    #[test]
    fn debug_trait() {
        assert_eq!(format!("{:?}", Alignment::Left), "Left");
        assert_eq!(format!("{:?}", Alignment::Center), "Center");
        assert_eq!(format!("{:?}", Alignment::Right), "Right");
    }
}
