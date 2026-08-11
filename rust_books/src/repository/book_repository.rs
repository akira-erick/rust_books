use crate::types::book::Book;

pub trait BookRepository {
    fn create(&mut self, book: Book);
    fn get(&self, id: u32) -> Option<&Book>;
    fn list(&self) -> &[Book];
    fn update(&mut self, book: Book) -> bool;
    fn delete(&mut self, id: u32) -> bool;
}