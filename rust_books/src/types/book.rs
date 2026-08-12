#[derive(Debug, Clone)]
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