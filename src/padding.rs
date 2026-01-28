#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Padding {
    pub left: usize,
    pub right: usize,
}

impl Padding {
    #[must_use]
    pub const fn new(left: usize, right: usize) -> Self {
        Self { left, right }
    }

    #[must_use]
    pub const fn uniform(padding: usize) -> Self {
        Self {
            left: padding,
            right: padding,
        }
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::uniform(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cases = [
            ((0, 0), Padding { left: 0, right: 0 }),
            ((1, 2), Padding { left: 1, right: 2 }),
            ((5, 3), Padding { left: 5, right: 3 }),
        ];
        for ((left, right), expected) in cases {
            assert_eq!(Padding::new(left, right), expected);
        }
    }

    #[test]
    fn uniform() {
        let cases = [(0, 0, 0), (1, 1, 1), (5, 5, 5)];
        for (value, expected_left, expected_right) in cases {
            let p = Padding::uniform(value);
            assert_eq!(p.left, expected_left);
            assert_eq!(p.right, expected_right);
        }
    }

    #[test]
    fn default_is_one() {
        let p = Padding::default();
        assert_eq!(p.left, 1);
        assert_eq!(p.right, 1);
    }

    #[test]
    fn copy_trait() {
        let padding = Padding::new(2, 3);
        let copied = padding;
        assert_eq!(padding, copied);
    }

    #[test]
    fn equality() {
        assert_eq!(Padding::new(1, 2), Padding::new(1, 2));
        assert_ne!(Padding::new(1, 2), Padding::new(2, 1));
    }
}
