use crate::emprestimo::Emprestimo;
use serde_json::{from_reader, to_writer};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

const EMPRESTIMO_ARQUIVO: &str = "emprestimos.json";

pub fn carregar_emprestimos() -> Vec<Emprestimo> {
    let file = File::open(EMPRESTIMO_ARQUIVO);
    match file {
        Ok(f) => {
            let reader = BufReader::new(f);
            from_reader(reader).unwrap_or_else(|_| {
                println!("Erro ao carregar empréstimos. Retornando lista vazia.");
                Vec::new()
            })
        }
        Err(_) => Vec::new(), // Arquivo não existe, retorna lista vazia
    }
}

pub fn salvar_emprestimos(emprestimos: &[Emprestimo]) -> io::Result<()> {
    let file = File::create(EMPRESTIMO_ARQUIVO)?;
    let writer = BufWriter::new(file);
    to_writer(writer, emprestimos)?;
    Ok(())
}
