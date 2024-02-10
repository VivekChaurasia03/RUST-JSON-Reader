use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Paragraph {
    name: String,
}

// This struct is written to follow the json structure that we have defined in the main function
#[derive(Deserialize, Serialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn read_json_type(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

fn main() {
    // We have just created a temporary json
    let json = r#"
    {
        "article": "How to to work with json in rust",
        "author": "Vivek",
        "paragraph": [
            {
                "name": "I am building a new Rust CLI tool to read JSON data."
            },
            {
                "name": "I have made few CLI tools in Rust."
            },
            {
                "name": "This is my third project and I will make few more. Thank you."
            }
        ]
    }"#;

    /* We will send the json to the function and we get back Article
    and we are going to print it with the help of parsed */
    let parsed: Article = read_json_type(json);
    println!("\n The name of the article is: {}", parsed.article);
    println!("\n The author of the article is: {}", parsed.author);
    println!(
        "\n The name of the first paragraph is: {}",
        parsed.paragraph[0].name
    );
}
