use crate::livro::Livro;
use serde_json::{from_reader, to_writer};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

const LIVRO_ARQUIVO: &str = "livros.json";

pub fn carregar_livros() -> Vec<Livro> {
    let file = File::open(LIVRO_ARQUIVO);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            from_reader(reader).unwrap_or_else(|_| {
                println!("Erro ao carregar livros. Retornando lista vazia.");
                Vec::new()
            })
        }
        Err(_) => Vec::new(), // Arquivo não existe, retorna lista vazia
    }
}

pub fn salvar_livros(livros: &[Livro]) -> io::Result<()> {
    let file = File::create(LIVRO_ARQUIVO)?;
    let writer = BufWriter::new(file);
    to_writer(writer, livros)?;
    Ok(())
}
