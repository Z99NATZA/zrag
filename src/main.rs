#[derive(Debug)]
struct Chunk {
    header: String,
    text: String,
}

fn chunk_content(content: &str) -> Vec<Chunk> {
    for line in content.lines() {
        println!("{:?}\n", line);
    }

    vec![Chunk {
        header: "".to_string(),
        text: "".to_string(),
    }]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("data/laravel-readme.md")?;
    let _chunks = chunk_content(&text);

    Ok(())
}
