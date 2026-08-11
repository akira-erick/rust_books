#[derive(Debug, Clone)]
struct book {
    id: u32,
    title: String,
    author: String,
    publisher: String,
    pages: u32,
}

impl Book {
    fn new(id: u32, title: String, author: String, publisher: String, pages: u32) -> Self {
        Book {
            id,
            title,
            author,
            publisher,
            pages,
        }
    }
}