use std::fmt;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead, Stdin};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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

    fn check_out(&mut self) -> Result<(), &str> {
        match self.status {
            Status::Available => {}
            _ => return Err("Book is not available!"),
        }
        self.status = Status::CheckedOut;
        return Ok(());
    }
}

enum LibrarySearchCriteria {
    Author,
    Title,
    ISBN,
}

impl LibrarySearchCriteria {
    fn matches<'a, T>(&self, book: &'a Book, value: T) -> bool
    where
        T: AsRef<str>,
    {
        match self {
            LibrarySearchCriteria::Author => book
                .author
                .to_lowercase()
                .contains(&value.as_ref().to_lowercase()),
            LibrarySearchCriteria::Title => book
                .title
                .to_lowercase()
                .contains(&value.as_ref().to_lowercase()),
            LibrarySearchCriteria::ISBN => book.isbn.eq(&value.as_ref()),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Library {
    books: Vec<Book>,
    owner: String,
}

impl Library {
    fn new(owner: &str) -> Library {
        Library {
            books: Vec::new(),
            owner: String::from(owner),
        }
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

    fn search_by<T>(&mut self, criteria: LibrarySearchCriteria, value: T) -> Option<&mut Book>
    where
        T: AsRef<str>,
    {
        return self.books.iter_mut().find(|x| criteria.matches(x, &value));
    }

    fn save(&self, path: &str) -> Result<(), io::Error> {
        let json = serde_json::to_string(&self).unwrap();
        return std::fs::write(path, json);
    }

    fn from_file(pathname: &str) -> Option<Library> {
        let path = Path::new(pathname);
        if path.exists() && path.is_file() {
            let s = read_to_string(path).unwrap();
            let library: Library = serde_json::from_str(&s).unwrap();
            return Some(library);
        } else {
            File::create(pathname).unwrap();
            return None;
        }
    }
}

fn initialize_demo(owner: &str) -> Library {
    let mut library: Library = Library::new(owner);
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

struct Config {
    library_path: String,
}

impl Config {
    fn new(library_path: &str) -> Config {
        Config {
            library_path: String::from(library_path),
        }
    }
}

fn read(stdin: &Stdin) -> String {
    return stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .expect("Failed to read input");
}

fn main() {
    let stdin = io::stdin();
    let config = Config::new("library.json");

    println!("Welcome to LMA, the library management app");
    println!("Loading library...");
    let lib = Library::from_file(&config.library_path);
    let mut library = match lib {
        Some(l) => l,
        None => {
            println!("Library file not found. Creating a new one...");
            println!("Who is the owner of this library?");
            let owner = read(&io::stdin());
            let l = initialize_demo(&owner);
            l.save(&config.library_path).expect("Couldn't create library");
            l
        }
    };

    println!("Welcome to {}'s library!", library.owner);

    loop {
        println!("Enter a title to search for: ");
        let title = read(&stdin);

        if title.is_empty() {
            println!("Title cannot be empty!");
            continue;
        }

        if let Some(book) = library.search_by(LibrarySearchCriteria::Title, &title) {
            println!("Here it is:");
            book.print();

            println!("Would you like to check out this book? (yes/no)");
            let answer = read(&stdin);
            match answer.to_lowercase().as_str() {
                "yes" | "y" => match book.check_out() {
                    Ok(_) => {
                        println!("Book checked out!");
                        library.save(&config.library_path).expect("Couldn't save library");
                    },
                    Err(e) => println!("{}", e),
                },
                _ => {}
            }
        } else {
            println!("Nothing found!");
        }

        println!("Would you like to search for another book? (yes/no)");
        let answer = read(&stdin);
        if answer.to_lowercase().as_str() != "yes" && answer.to_lowercase().as_str() != "y" {
            break;
        }
    }

    println!("Saving library...");
    library.save(&config.library_path).expect("Couldn't save library");
    println!("Goodbye!");
}
