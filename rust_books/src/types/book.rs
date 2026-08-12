use std::fmt;

#[derive(Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub publisher: String,
    pub pages: u32,
}

impl Book {
    pub fn new(title: String, author: String, publisher: String, pages: u32) -> Self {
        Book {
            id: 0,
            title,
            author,
            publisher,
            pages,
        }
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} - {} ({}) - {} páginas",
            self.id,
            self.title,
            self.author,
            self.publisher,
            self.pages
        )
    }
}