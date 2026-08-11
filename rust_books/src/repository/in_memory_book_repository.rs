use crate::types::book::Book;
use crate::repository::book_repository::BookRepository;

pub struct InMemoryBookRepository {
    books: Vec<Book>,
}

impl InMemoryBookRepository {
    pub fn new() -> Self {
        Self {
            books: Vec::new(),
        }
    }
}

impl BookRepository for InMemoryBookRepository {
    fn create(&mut self, book: Book) {
        self.books.push(book);
    }

    fn get(&self, id: u32) -> Option<&Book> {
        self.books.iter().find(|book| book.id == id)
    }

    fn list(&self) -> Vec<&Book> {
        self.books.iter().collect()
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