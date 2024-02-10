# Rust JSON Reader CLI

Welcome to the Rust JSON Reader CLI tool! This project is a small and efficient command-line tool built in Rust for reading JSON data. It utilizes the serde and serde_json crates for handling JSON deserialization.

## How to Use

To use the tool, follow these steps:

1. Clone the repository to your local machine.
2. Navigate to the project directory in your terminal.
3. Run the following command:

   ```bash
   cargo run
   ```

4. The tool will read the predefined JSON data and display the article details.

## Code Overview

### Dependencies

The project relies on the following crates:

- serde: For serialization and deserialization of Rust data structures.
- serde_json: For JSON parsing and handling.

### Code Structure

- `Paragraph` Struct: Represents a paragraph within an article.

- `Article` Struct: Represents an article containing an article name, author, and a vector of paragraphs.

- `read_json_type` Function: Accepts raw JSON data and returns a deserialized `Article` structure.

- `main` Function: Demonstrates the usage of the tool by providing a sample JSON and printing article details.

## Example

```bash
cargo run
```

Output:

```plaintext
The name of the article is: How to to work with json in rust

The author of the article is: Vivek

The name of the first paragraph is: I am building a new Rust CLI tool to read JSON data.
```

Feel free to explore and modify this Rust JSON Reader CLI Tool. Happy coding!

---
