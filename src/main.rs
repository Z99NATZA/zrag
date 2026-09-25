use fastembed::{TextEmbedding, EmbeddingModel, TextInitOptions};
use zrag::AppResult;

#[derive(Debug)]
struct Chunk {
    header: String,
    text: String,
    embeddings: Vec<f32>,
}

fn chunk(content: &str, model: &mut TextEmbedding) -> AppResult<Vec<Chunk>> {
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut text = String::new();
    let mut header = String::new();

    for line in content.lines() {
        if let Some(header_raw) = line.strip_prefix("## ") {
            if !text.is_empty() {
                chunks.push(Chunk {
                    header: header.to_owned(),
                    text: text.to_owned(),
                    embeddings: embed(model, header, std::mem::take(&mut text))?
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
        header: header.to_owned(),
        text: text.to_owned(),
        embeddings: embed(model, header, std::mem::take(&mut text))?
    });

    Ok(chunks)
}

fn main() -> AppResult<()> {
    let text = std::fs::read_to_string("data/laravel-readme.md")?;
    let mut model = TextEmbedding::try_new(
        TextInitOptions::new(EmbeddingModel::AllMiniLML6V2)
    )?;
    let chunks = chunk(&text, &mut model);

    println!("{chunks:#?}");

    Ok(())
}

fn embed(
    model: &mut TextEmbedding,
    header: String,
    text: String
) -> AppResult<Vec<f32>> {
    let content = format!("{header}\n{}", &text);

    let embeddings = model.embed(
        vec![content],
        None,
    )?;

    Ok(embeddings.into_iter().next().unwrap())
}


