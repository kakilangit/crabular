#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

impl core::str::FromStr for Alignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "left" | "l" => Ok(Alignment::Left),
            "center" | "c" | "middle" => Ok(Alignment::Center),
            "right" | "r" => Ok(Alignment::Right),
            _ => Err(()),
        }
    }
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

    #[test]
    fn from_str() {
        assert_eq!("left".parse(), Ok(Alignment::Left));
        assert_eq!("l".parse(), Ok(Alignment::Left));
        assert_eq!("center".parse(), Ok(Alignment::Center));
        assert_eq!("c".parse(), Ok(Alignment::Center));
        assert_eq!("middle".parse(), Ok(Alignment::Center));
        assert_eq!("right".parse(), Ok(Alignment::Right));
        assert_eq!("r".parse(), Ok(Alignment::Right));
        assert_eq!("invalid".parse::<Alignment>(), Err(()));
    }
}
