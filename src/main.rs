#[derive(Debug)]
struct Chunk {
    header: String,
    text: String,
}

fn chunk(content: &str) -> Vec<Chunk> {
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut text = String::new();
    let mut header = String::new();

    for line in content.lines() {
        if let Some(header_raw) = line.strip_prefix("## ") {
            if !text.is_empty() {
                chunks.push(Chunk {
                    header,
                    text: std::mem::take(&mut text),
                });
            }

            header = header_raw.to_owned();
        } else {
            if !text.is_empty() {
                text.push('\n');
            }

            text.push_str(line);
        }
    }

    chunks.push(Chunk {
        header,
        text: std::mem::take(&mut text),
    });

    chunks
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("data/laravel-readme.md")?;
    let chunks = chunk(&text);
    println!("{chunks:#?}");

    Ok(())
}
