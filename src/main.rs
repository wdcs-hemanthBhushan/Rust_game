use std::fmt::Display;
use std::io;

enum Options {
    AddBook,
    UpdateBook,
    SearchBook,
    RemoveBook,
    TotalValue,
    Quit,
}
struct Book<T, U, V> {
    title: T,
    author: T,
    edition: U,
    price: V,
}

impl<T, U, V> PrintDetails<T, U, V> for Book<T, U, V>
where
    T: Display,
    U: Display,
    V: Display,
{
    fn new(title: T, author: T, edition: U, price: V) -> Self {
        Self {
            title,
            author,
            edition,
            price,
        }
    }

    fn show_details(&self) {
        println!(
            "details are  : {},{},{},{}",
            self.author, self.edition, self.price, self.price
        );
    }
}

trait PrintDetails<T, U, V> {
    fn new(title: T, author: T, gener: U, price: V) -> Self;
    fn show_details(&self);
}

struct Inventory {
    inventory: Vec<Book<String, f64, u32>>,
}

impl Inventory {
    fn build_new_inventory() -> Self {
        Self {
            inventory: Vec::new(),
        }
    }
    fn add_product(&mut self, book: Book<String, f64, u32>) -> Result<(), &'static str> {
        if self.product_exists(&book.title) {
            return Err("Product with the same name already exists.");
        }
        self.inventory.push(book);
        Ok(())
    }

    fn product_exists(&self, title: &str) -> bool {
        self.inventory.iter().any(|x| x.title == title)
    }
}

fn discount<T, U, V>(item: &impl PrintDetails<T, U, V>, percentage: f64) -> f64 {
    22.2
}

fn main() {
    let mut inventory = Inventory::build_new_inventory();

    loop {
        println!("Choose an operation:");
        println!("1. Add Book");
        println!("2. Update Book");
        println!("3. Search Book");
        println!("4. Remove Book");
        println!("5. Total Books Value");
        println!("6. Quit");

        let choice: u32 = match get_user_input("enter the Option").trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        let option: Options = match choice {
            1 => Options::AddBook,
            2 => Options::UpdateBook,
            3 => Options::SearchBook,
            4 => Options::RemoveBook,
            5 => Options::TotalValue,
            6 => Options::Quit,
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 7.");
                continue;
            }
        };

        match option {
            Options::AddBook => {
                println!("AddBook");
                let book: Book<String, f64, u32> = get_product_details();

                match inventory.add_product(book) {
                    Ok(()) => println!("Book added successfully"),
                    Err(err) => println!("Error: {}", err),
                }
            }
            Options::UpdateBook => {
                println!("UpdateBook");
            }
            Options::SearchBook => {
                println!("SearchBook");
            }
            Options::RemoveBook => {
                println!("RemoveBook");
            }
            Options::TotalValue => {
                println!("TotalValue");
            }
            Options::Quit => {
                println!("Quit");
            }
        }
    }
}
fn get_product_details() -> Book<String, f64, u32> {
    let title: String = get_user_input("enter the title");
    let author: String = get_user_input("enter the author");
    let gener: f64 = get_user_input("edition")
        .parse()
        .expect("Invalid Input for edition");
    let price: u32 = get_user_input("Price")
        .parse()
        .expect("Invalid Input for Price");

    Book::new(title, author, gener, price)
}
fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input: String = String::new();

    io::stdin().read_line(&mut input);

    input.trim().to_owned()
}
