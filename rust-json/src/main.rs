use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}


fn main() {
    let json = r#"
    {
    "article": "How to work with JSON in RUST",
    "author": "Abhishek",
    "paragraph": [
    {
    "name": "Header"
    },
    {
    "name": "body"
    },
    {
    "name": "conclusion"
    }
    ]
    }
    "#;

    let parsed: Article = read_json_types(json);
    println!("\n\n The name of the first paragraph is : {}", parsed.paragraph[0].name);
}

fn read_json_types(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return  parsed;
}
