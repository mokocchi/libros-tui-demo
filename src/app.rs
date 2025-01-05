use crate::library::{Book, Config, Library, LibrarySearchCriteria};

#[derive(Debug)]
pub enum CurrentScreen {
    Loading,
    Home,
    NewOwner,
    Searching,
    CheckingOut,
    CheckedOutResult,
    Exiting,
}

#[derive(Debug)]
pub struct App {
    pub loaded: bool,
    pub current_screen: CurrentScreen,
    pub library: Option<Library>,
    pub config: Config,
    pub entering_owner: bool,
    pub owner_input: String,
    pub searching_criteria: LibrarySearchCriteria,
    pub searching_input: String,
    pub term_input_mode: bool,
    pub selected_book: Option<Book>,
    pub checkout_success: Option<Result<(), String>>,
    pub error_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let config = Config::new("library.json");
        App {
            loaded: false,
            current_screen: CurrentScreen::Loading,
            library: None,
            entering_owner: false,
            owner_input: String::new(),
            config,
            searching_criteria: LibrarySearchCriteria::Title,
            searching_input: String::new(),
            term_input_mode: false,
            checkout_success: None,
            error_message: None,
            selected_book: None,
        }
    }

    fn loaded(&mut self) {
        self.loaded = true
    }

    pub fn load(&mut self) {
        let lib = Library::from_file(&self.config.library_path);
        self.library = match lib {
            Some(l) => {
                self.loaded();
                Some(l)
            }
            None => {
                self.entering_owner = true;
                None
            }
        };
    }

    pub fn apply_search(&mut self) {
        if self.searching_input.is_empty() {
            return;
        }
        self.selected_book = match self
            .library
            .as_ref()
            .unwrap()
            .search_by(&self.searching_criteria, &self.searching_input)
        {
            Some(book) => Some(book.clone()),
            None => None,
        }
    }

    pub fn check_out(&mut self) {
        self.checkout_success = match self
            .library
            .as_mut()
            .unwrap()
            .check_out(self.selected_book.as_ref().unwrap().get_isbn())
        {
            Ok(_) => Some(Ok(())),
            Err(e) => Some(Err(e.to_string())),
        };
    }

    pub fn initialize_demo(&mut self) {
        self.library = Some(Library::initialize_demo(&self.owner_input));
        self.library
            .as_ref()
            .unwrap()
            .save(&self.config.library_path)
            .expect("Couldn't create library"); //todo: handle error
        self.loaded();
    }
}
