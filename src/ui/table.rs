use crate::{channel::Channel, ui::entry::Entry};

#[derive(Clone)]
pub struct Table {
    left: Channel,
    right: Channel,
    elements: Vec<String>,
}

impl Table {
    pub fn new(left: Channel, right: Channel) -> Table {
        Table {
            left,
            right,
            elements: vec![table_head()],
        }
    }

    pub fn add(mut self, title: &str, mapping_fn: fn(Channel) -> Entry) -> Table {
        let left = mapping_fn(self.left);
        let right = mapping_fn(self.right);
        let new_row = table_row(title, &left.formatted(), &right.formatted());
        self.elements.push(new_row);

        self
    }

    pub fn set_headers(mut self, title: &str, left: &str, right: &str) -> Table {
        let new_row = table_row(title, left, right);
        self.elements.push(new_row);

        self
    }

    pub fn add_section(mut self) -> Table {
        self.elements.push(table_section());

        self
    }

    pub fn build(&mut self) -> String {
        self.elements.push(table_footer());
        self.elements.join("\n")
    }
}

fn table_head() -> String {
    format!("┌{}┬{}┬{}┐", "─".repeat(26), "─".repeat(20), "─".repeat(20))
}

fn table_footer() -> String {
    format!("└{}┴{}┴{}┘", "─".repeat(26), "─".repeat(20), "─".repeat(20))
}

fn table_section() -> String {
    format!("├{}┼{}┼{}┤", "─".repeat(26), "─".repeat(20), "─".repeat(20))
}

fn table_row(title: &str, left: &str, right: &str) -> String {
    format!("│  {:<22}  │    {:>14}  │    {:>14}  │", title, left, right)
}
