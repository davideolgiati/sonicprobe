use crate::ui::entry::Entry;

#[derive(Clone)]
pub struct Table{
        elements: Vec<String>
}

impl Table {
        pub fn new() -> Table {
                Table { 
                        elements: vec![table_head()]
                }
        }

        pub fn add(mut self, title: Entry, left: Entry, right: Entry) -> Table {
                let new_row = table_row(&title.value(), &left.value(), &right.value());
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
        format!("┌{}┬{}┬{}┐","─".repeat(28),"─".repeat(19),"─".repeat(20))
}

fn table_footer() -> String {
        format!( "└{}┴{}┴{}┘\n\n", "─".repeat(28), "─".repeat(19), "─".repeat(20))
}

fn table_section() -> String {
        format!("├{}┼{}┼{}┤", "─".repeat(28), "─".repeat(19), "─".repeat(20))
}

fn table_row(title: &str, left: &str, right: &str) -> String {
        format!("│  {:<23}   │    {:>12}   │     {:>12}   │", title, left, right)
}