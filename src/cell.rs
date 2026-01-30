use crate::Alignment;

#[derive(Clone)]
pub struct Cell {
    content: String,
    alignment: Alignment,
    span: usize,
}

impl Cell {
    #[must_use]
    pub fn new(content: &str, alignment: Alignment) -> Self {
        Self {
            content: content.to_string(),
            alignment,
            span: 1,
        }
    }

    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    #[must_use]
    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    #[must_use]
    pub fn span(&self) -> usize {
        self.span
    }

    pub fn set_span(&mut self, span: usize) {
        self.span = span.max(1);
    }

    pub fn set_alignment(&mut self, alignment: Alignment) {
        self.alignment = alignment;
    }
}

impl core::fmt::Display for Cell {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let cases = [
            ("hello", Alignment::Left),
            ("world", Alignment::Center),
            ("test", Alignment::Right),
            ("", Alignment::Left),
        ];
        for (content, alignment) in cases {
            let cell = Cell::new(content, alignment);
            assert_eq!(cell.content(), content);
            assert_eq!(cell.alignment(), alignment);
            assert_eq!(cell.span(), 1);
        }
    }

    #[test]
    fn set_span() {
        let cases = [1, 2, 3, 5, 10];
        for span in cases {
            let mut cell = Cell::new("test", Alignment::Left);
            cell.set_span(span);
            assert_eq!(cell.span(), span);
        }
    }

    #[test]
    fn set_alignment() {
        let mut cell = Cell::new("test", Alignment::Left);

        cell.set_alignment(Alignment::Right);
        assert_eq!(cell.alignment(), Alignment::Right);

        cell.set_alignment(Alignment::Center);
        assert_eq!(cell.alignment(), Alignment::Center);
    }

    #[test]
    fn clone_trait() {
        let cell = Cell::new("test", Alignment::Center);
        let cloned = cell.clone();
        assert_eq!(cell.content(), cloned.content());
        assert_eq!(cell.alignment(), cloned.alignment());
        assert_eq!(cell.span(), cloned.span());
    }

    #[test]
    fn span_preserved_on_clone() {
        let mut cell = Cell::new("test", Alignment::Left);
        cell.set_span(3);
        let cloned = cell.clone();
        assert_eq!(cloned.span(), 3);
    }

    #[test]
    fn display_trait_content() {
        let cell = Cell::new("hello world", Alignment::Left);
        let displayed = format!("{cell}");
        assert_eq!(displayed, "hello world");
    }

    #[test]
    fn display_trait_empty() {
        let cell = Cell::new("", Alignment::Center);
        let displayed = format!("{cell}");
        assert_eq!(displayed, "");
    }

    #[test]
    fn display_trait_unicode() {
        let cell = Cell::new("日本語", Alignment::Right);
        let displayed = format!("{cell}");
        assert_eq!(displayed, "日本語");
    }
}
