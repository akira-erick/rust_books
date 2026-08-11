#[derive(Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub publisher: String,
    pub pages: u32,
}

impl Book {
    pub fn new(id: u32, title: String, author: String, publisher: String, pages: u32) -> Self {
        Book {
            id,
            title,
            author,
            publisher,
            pages,
        }
    }
}