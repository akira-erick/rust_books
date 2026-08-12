use crate::repository::book_repository::BookRepository;
use crate::types::book::Book;

pub enum BookSort {
    Id,
    Title,
    Author,
    Publisher,
    Pages,
}

pub struct BookService<R: BookRepository> {
    repository: R,
}

impl<R: BookRepository> BookService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn create_book(&mut self, book: Book) -> u32 {
        self.repository.create(book)
    }

    pub fn get_book(&self, id: u32) -> Option<&Book> {
        self.repository.get(id)
    }

    pub fn get_books(&self) -> &[Book] {
        self.repository.list()
    }

    pub fn update_book(&mut self, book: Book) -> Result<(), String> {
        if !self.repository.update(book) {
            return Err(String::from("Book not found"));
        }

        Ok(())
    }


    pub fn delete_book(&mut self, id: u32) -> Result<(), String> {
        if !self.repository.delete(id) {
            return Err(String::from("Book not found"));
        }

        Ok(())
    }

    pub fn get_books_sorted (&self, sort: BookSort) -> Vec<&Book> {
        let mut books: Vec<&Book> = self.repository
            .list()
            .iter()
            .collect();

        match sort {
            BookSort::Id => books.sort_by(|a, b| a.id.cmp(&b.id)),
            BookSort::Title => books.sort_by(|a, b| a.title.cmp(&b.title)),
            BookSort::Author => books.sort_by(|a, b| a.author.cmp(&b.author)),
            BookSort::Publisher => books.sort_by(|a, b| a.publisher.cmp(&b.publisher)),
            BookSort::Pages => books.sort_by(|a, b| a.pages.cmp(&b.pages)),
        }

        books
    }
}