Absolutely, let's enhance the question by incorporating generics into the structs. 

### Revised Question: Book Inventory Management with Generics

You are tasked with implementing a sophisticated inventory management system for a bookstore, leveraging advanced Rust features. The system should handle books of different genres, authors, and prices, allowing for flexibility with data types. Implement the following:

1. **Define Traits:**
   - Create a trait called `PrintDetails` with a method `print_details(&self)` to print details of an item.

2. **Create Generic Structs:**
   - Define a generic struct `Book<T, U, V>` that represents a book with fields for title, author, genre, and price. The types `T`, `U`, and `V` should allow for flexible data types for title, author, and price, respectively.

3. **Generics and Bounded Functions:**
   - Implement a function called `discount<T: PrintDetails>(item: T, percentage: f64) -> f64` that takes an item implementing `PrintDetails` and returns the discounted price after applying the given percentage.

4. **Collections and Iterations:**
   - Create a bookstore inventory using a vector. Populate it with instances of generic books of different genres, authors, and prices.

5. **Closures:**
   - Write a closure that takes a genre as an argument and filters books in the inventory by that genre.

6. **Usage Example:**
   - Use the defined traits, generic structs, and functions to demonstrate the following:
     - Print details of each generic book in the inventory.
     - Apply a 10% discount to all books and print the discounted prices.
     - Filter and print details of all generic books in a specific genre.

This revised question incorporates generics into the struct definition, allowing candidates to showcase their understanding of generic types in Rust.