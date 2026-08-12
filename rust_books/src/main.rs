mod types;
mod repository;
mod services;

use crate::repository::book_repository::BookRepository;
use crate::repository::in_memory_book_repository::InMemoryBookRepository;
use crate::services::book_service::{BookService, BookSort};

fn main() {
    let mut repository = InMemoryBookRepository::new();
    let mut service = BookService::new(repository);

    let mut input = String::new();

    println!("CRUD de Livros");

    println!("1 - Inserir livro");
    println!("2 - Alterar livro");
    println!("3 - Apagar livro");
    println!("4 - Exibir livro");
    println!("5 - Listar livros");
    println!("0 - Sair");

    println!("Escolha uma opção: ");

    std::io::stdin().read_line(&mut input).unwrap();

    let choice = input.trim().parse::<u32>().unwrap_or(0);

    match choice {
        1 => {
            println!("Inserir livro");
            insert_book(&mut service);
        }
        2 => {
            println!("Alterar livro");
            update_book(&mut service);
        }
        3 => {
            println!("Apagar livro");
            delete_book(&mut service);
        }
        4 => {
            println!("Exibir livro");
            display_book(&service);
        }
        5 => {
            println!("Listar livros");
            list_books(&service);
        }
        0 => {
            println!("Saindo...");
        }
        _ => {
            println!("Opção inválida");
        }
    }
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_number(prompt: &str) -> u32 {
    loop {
        let input = read_input(prompt);

        match input.parse::<u32>() {
            Ok(number) => return number,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn insert_book<R: BookRepository>(service: &mut BookService<R>) {
    let title = read_input("Título: ");
    let author = read_input("Autor: ");
    let publisher = read_input("Editora: ");
    let pages = read_number("Número de páginas: ");

    let book = types::book::Book::new(title, author, publisher, pages);

    let book_id = service.create_book(book);

    println!("Livro inserido com ID: {}", book_id);
}

fn update_book<R: BookRepository>(service: &mut BookService<R>) {
    let id = read_number("ID do livro a ser alterado: ");
    let title = read_input("Novo título: ");
    let author = read_input("Novo autor: ");
    let publisher = read_input("Nova editora: ");
    let pages = read_number("Novo número de páginas: ");

    let book = types::book::Book {
        id,
        title,
        author,
        publisher,
        pages,
    };

    match service.update_book(book) {
        Ok(_) => println!("Livro atualizado com sucesso."),
        Err(err) => println!("Erro ao atualizar livro: {}", err),
    }
}

fn delete_book<R: BookRepository>(service: &mut BookService<R>) {
    let id = read_number("ID do livro a ser apagado: ");

    match service.delete_book(id) {
        Ok(_) => println!("Livro apagado com sucesso."),
        Err(err) => println!("Erro ao apagar livro: {}", err),
    }
}

fn display_book<R: BookRepository>(service: &BookService<R>) {
    let id = read_number("ID do livro a ser exibido: ");

    match service.get_book(id) {
        Some(book) => println!("Livro encontrado: {:?}", book),
        None => println!("Livro não encontrado."),
    }
}

fn list_books<R: BookRepository>(service: &BookService<R>) {
    let sort_option = read_number("Escolha a opção de ordenação (1 - Título, 2 - Autor, 3 - Editora, 4 - Número de páginas): ");

    let sort = match sort_option {
       1 => BookSort::Id,
       2 => BookSort::Title,
       3 => BookSort::Author,
       4 => BookSort::Publisher,
       5 => BookSort::Pages,
       _ => {
           println!("Opção inválida. Ordenando por ID.");
           BookSort::Id
       }
    };

    let books = service.get_books_sorted(sort);

    if books.is_empty() {
        println!("Nenhum livro encontrado.");
    } else {
        for book in books {
            println!("{:?}", book);
        }
    }
}