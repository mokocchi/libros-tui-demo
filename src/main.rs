use std::fmt;
use std::io::{self, BufRead};

enum Genre {
    Fiction,
    NonFiction,
    ScienceFiction,
    Mystery,
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Genre::Fiction => write!(f, "Fiction"),
            Genre::NonFiction => write!(f, "Non-Fiction"),
            Genre::ScienceFiction => write!(f, "Science Fiction"),
            Genre::Mystery => write!(f, "Mystery"),
        }
    }
}

enum Status {
    Available,
    CheckedOut,
    Lost,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Available => write!(f, "Available"),
            Status::CheckedOut => write!(f, "Checked Out"),
            Status::Lost => write!(f, "Lost"),
        }
    }
}

struct Book {
    title: String,
    author: String,
    isbn: String,
    publication_year: u16,
    genre: Genre,
    status: Status,
}

impl Book {
    fn new(title: &str, author: &str, isbn: &str, publication_year: u16, genre: Genre) -> Book {
        Book {
            title: String::from(title),
            author: String::from(author),
            isbn: String::from(isbn),
            publication_year,
            genre,
            status: Status::Available,
        }
    }

    fn print(&self) {
        println!("Title: {}", self.title);
        println!("Author: {}", self.author);
        println!("ISBN: {}", self.isbn);
        println!("Genre: {}", self.genre);
        println!("Published in {}", self.publication_year);
        println!("({})", self.status);
    }
}

enum LibrarySearchCriteria {
    Author,
    Title,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn add(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print(&self) {
        for book in &self.books {
            println!("======");
            book.print();
        }
        println!("======");
    }

    fn search_by<T>(&self, criteria: LibrarySearchCriteria, value: T) -> Option<&Book>
    where
        T: AsRef<str>,
    {
        return self.books.iter().find(|x| match criteria {
            LibrarySearchCriteria::Author => x
                .author
                .to_lowercase()
                .contains(&value.as_ref().to_lowercase()),
            LibrarySearchCriteria::Title => x
                .title
                .to_lowercase()
                .contains(&value.as_ref().to_lowercase()),
        });
    }
}

fn initialize() -> Library {
    let mut library: Library = Library::new();
    library.add(Book::new(
        "The Great Gatsby",
        "F. Scott Fitzgerald",
        "9780743273565",
        1925,
        Genre::Fiction,
    ));
    library.add(Book::new(
        "To Kill a Mockingbird",
        "Harper Lee",
        "9780061120084",
        1960,
        Genre::Fiction,
    ));
    library.add(Book::new(
        "1984",
        "George Orwell",
        "9780451524935",
        1949,
        Genre::ScienceFiction,
    ));
    return library;
}

fn main() {
    let library: Library = initialize();
    library.print();
    let stdin = io::stdin();
    println!("Welcome to Jose's library!");
    println!("Enter a title to search for: ");

    let input = stdin.lock().lines().next().expect("Failed to read");

    let title = input.expect("Failed to read input");

    match library.search_by(LibrarySearchCriteria::Title, &title) {
        None => println!("Nothing found!"),
        Some(b) => {
            println!("Here it is!");
            b.print()
        }
    }
}
