#[derive(Debug)]
struct Chunk {
    header: String,
    text: String,
}

fn chunk_content(content: &str) -> Vec<Chunk> {
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut text = String::new();
    let mut header = String::new();

    for line in content.lines() {
        if is_header(line) {
            if !text.is_empty() {
                chunks.push(
                    Chunk {
                        header,
                        text: text.clone(),
                    }
                );

                text = String::new();
            }

            header = line.to_string();
        }
        else {
            text = format!("{}. {}", text, line);
        }
    }

    chunks.push(
        Chunk {
            header,
            text: text.clone(),
        }
    );

    chunks
}

fn is_header(value: &str) -> bool {
    if value.starts_with("## ") {
        return true;
    }

    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("data/laravel-readme.md")?;
    let chunks = chunk_content(&text);
    println!("{chunks:#?}");

    Ok(())
}
