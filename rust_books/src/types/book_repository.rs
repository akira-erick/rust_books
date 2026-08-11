trait BookRepository {
    fn create(&mut self, book: Book);
    fn get(&self, id: u32) -> Option<&Book>;
    fn list(&self) -> Vec<&Book>;
    fn update(&mut self, book: Book) -> bool;
    fn delete(&mut self, id: u32) -> bool;
}