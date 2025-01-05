use std::fmt;
use std::fs::{read_to_string, File};
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
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

    pub fn get_title(&self) -> &str {
        return &self.title;
    }

    pub fn get_author(&self) -> &str {
        return &self.author;
    }

    pub fn get_isbn(&self) -> &str {
        return &self.isbn;
    }

    pub fn get_available(&self) -> bool {
        return self.status == Status::Available;
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

#[derive(Debug)]
pub enum LibrarySearchCriteria {
    Author,
    Title,
    ISBN,
}

impl fmt::Display for LibrarySearchCriteria {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibrarySearchCriteria::Author => write!(f, "Author"),
            LibrarySearchCriteria::Title => write!(f, "Title"),
            LibrarySearchCriteria::ISBN => write!(f, "ISBN"),
        }
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
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

    pub fn get_owner(&self) -> &str {
        return &self.owner;
    }

    fn add(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn search_by<T>(&self, criteria: &LibrarySearchCriteria, value: T) -> Option<&Book>
    where
        T: AsRef<str>,
    {
        return self.books.iter().find(|x| criteria.matches(x, &value));
    }

    pub fn save(&self, path: &str) -> Result<(), io::Error> {
        let json = serde_json::to_string(&self).unwrap();
        return std::fs::write(path, json);
    }

    pub fn from_file(pathname: &str) -> Option<Library> {
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

    pub fn check_out(&mut self, isbn: &str) -> Result<(), &str> {
        match self.books.iter_mut().find(|x| x.isbn.eq(isbn)) {
            Some(book) => book.check_out(),
            None => Err("Book not found!"),
        }
    }

    pub fn initialize_demo(owner: &str) -> Library {
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

    pub fn get_books(&self) -> &Vec<Book> {
        return &self.books;
    }
}

#[derive(Debug)]
pub struct Config {
    pub library_path: String,
}

impl Config {
    pub fn new(library_path: &str) -> Config {
        Config {
            library_path: String::from(library_path),
        }
    }
}
