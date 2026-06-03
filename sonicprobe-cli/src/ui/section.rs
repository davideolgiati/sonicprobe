use crate::{ui::entry::Entry};

#[derive(Clone, Copy)]
pub enum Layout {
    Text,
    Signal,
    Stereo,
}

impl Layout {
    fn render(self, value: &str, unit: Option<&str>) -> String {
        match self {
            Self::Text => value.to_owned(),
            Self::Signal => format!("{value:<5}{}", unit.unwrap_or_default()),
            Self::Stereo => match unit {
                Some(unit) => format!("{value:>7} {unit}"),
                None => format!("{value:>7}"),
            },
        }
    }
}

#[derive(Clone)]
pub struct Section {
    elements: Vec<String>,
    layout: Layout,
}

impl Section {
    pub fn new(title: &str, layout: Layout) -> Self {
        Self {
            elements: vec![section_header(title)],
            layout,
        }
    }

    #[inline]
    pub fn add(mut self, title: &str, entry: Entry) -> Self {
        let (value, unit) = entry.into_parts();
        let new_line = format!("   {:<18} :  {}", title, self.layout.render(&value, unit.as_deref()));
        self.elements.push(new_line);

        self
    }

    pub fn build(&mut self) -> String {
        self.elements.push("\n\n".to_owned());
        self.elements.join("\n")
    }
}

fn section_header(title: &str) -> String {
    let separator_len = 70 - title.len() - 4;
    format!(
        "── {} {}\n",
        title.to_ascii_uppercase(),
        "─".repeat(separator_len)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::entry::Entry;

    fn line(layout: Layout, title: &str, entry: Entry) -> String {
        Section::new("X", layout).add(title, entry).build()
    }

    #[test]
    fn text_left_flush() {
        assert!(line(Layout::Text, "Filename", Entry::from("01 - American Idiot".to_owned()))
            .contains("   Filename           :  01 - American Idiot"));
    }

    #[test]
    fn signal_plain_number() {
        assert!(line(Layout::Signal, "Sample Count", Entry::from(15375024usize))
            .contains("   Sample Count       :  15375024"));
    }

    #[test]
    fn signal_rate_unit_aligned() {
        assert!(line(Layout::Signal, "Sample Rate", Entry::from_rate("44.1"))
            .contains("   Sample Rate        :  44.1 kHz"));
    }

    #[test]
    fn signal_bit_unit_aligned() {
        assert!(line(Layout::Signal, "Bit Depth", Entry::from_bit(16))
            .contains("   Bit Depth          :  16   bit"));
    }

    #[test]
    fn stereo_no_unit_right() {
        assert!(line(Layout::Stereo, "Channels", Entry::from(2usize))
            .contains("   Channels           :        2"));
    }

    #[test]
    fn stereo_percent_right() {
        assert!(line(Layout::Stereo, "Stereo Correlation", Entry::from_percent(72.85))
            .contains("   Stereo Correlation :   +72.85 %"));
    }
}