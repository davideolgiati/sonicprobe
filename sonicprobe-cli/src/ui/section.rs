use crate::{ui::entry::Entry};

#[derive(Clone)]
pub struct Section {
    elements: Vec<String>,
}

impl Section {
    pub fn new(title: &str) -> Self {
        Self {
            elements: vec![section_header(title)],
        }
    }

    #[inline]
    pub fn add(mut self, title: &str, entry: Entry) -> Self {
        let new_line = format!("   {:<18} : {}", title, entry.formatted());
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