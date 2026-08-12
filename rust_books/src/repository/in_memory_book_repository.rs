use crate::types::book::Book;
use crate::repository::book_repository::BookRepository;

pub struct InMemoryBookRepository {
    books: Vec<Book>,
    next_id: u32,
}

impl InMemoryBookRepository {
    pub fn new() -> Self {
        Self {
            books: Vec::new(),
            next_id: 1,
        }
    }
}

impl BookRepository for InMemoryBookRepository {
    fn create(&mut self, mut book: Book) -> u32 {
        let id = self.next_id;

        book.id = id;
        self.next_id += 1;

        self.books.push(book);

        id
    }

    fn get(&self, id: u32) -> Option<&Book> {
        self.books.iter().find(|book| book.id == id)
    }

    fn list(&self) -> &[Book] {
        &self.books
    }

    fn update(&mut self, book: Book) -> bool {
        if let Some(existing) = self.books.iter_mut().find(|b| b.id == book.id) {
            *existing = book;
            true
        } else {
            false
        }
    }

    fn delete(&mut self, id: u32) -> bool {
        if let Some(index) = self.books.iter().position(|book| book.id == id) {
            self.books.remove(index);
            true
        } else {
            false
        }
    }
}