#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WidthConstraint {
    Auto,
    Fixed(usize),
    Min(usize),
    Max(usize),
    Proportional(u8),
    Wrap(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_equality() {
        let cases = [
            (WidthConstraint::Auto, WidthConstraint::Auto, true),
            (WidthConstraint::Fixed(10), WidthConstraint::Fixed(10), true),
            (
                WidthConstraint::Fixed(10),
                WidthConstraint::Fixed(20),
                false,
            ),
            (WidthConstraint::Min(5), WidthConstraint::Min(5), true),
            (WidthConstraint::Max(20), WidthConstraint::Max(20), true),
            (
                WidthConstraint::Proportional(50),
                WidthConstraint::Proportional(50),
                true,
            ),
            (WidthConstraint::Wrap(10), WidthConstraint::Wrap(10), true),
            (WidthConstraint::Auto, WidthConstraint::Fixed(10), false),
        ];
        for (a, b, expected) in cases {
            assert_eq!(a == b, expected);
        }
    }

    #[test]
    fn clone_trait() {
        let cases = [
            WidthConstraint::Auto,
            WidthConstraint::Fixed(10),
            WidthConstraint::Min(5),
            WidthConstraint::Max(20),
            WidthConstraint::Proportional(50),
            WidthConstraint::Wrap(15),
        ];
        for constraint in cases {
            let cloned = constraint.clone();
            assert_eq!(constraint, cloned);
        }
    }

    #[test]
    fn debug_trait() {
        assert_eq!(format!("{:?}", WidthConstraint::Auto), "Auto");
        assert_eq!(format!("{:?}", WidthConstraint::Fixed(10)), "Fixed(10)");
        assert_eq!(format!("{:?}", WidthConstraint::Min(5)), "Min(5)");
        assert_eq!(format!("{:?}", WidthConstraint::Max(20)), "Max(20)");
        assert_eq!(
            format!("{:?}", WidthConstraint::Proportional(50)),
            "Proportional(50)"
        );
        assert_eq!(format!("{:?}", WidthConstraint::Wrap(15)), "Wrap(15)");
    }
}
