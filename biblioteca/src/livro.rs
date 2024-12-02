use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// Enum para o gênero do livro
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Genero {
    None, // Valor padrão
    Ficcao,
    NaoFiccao,
    Fantasia,
    Ciencia,
    Romance,
}

// Estrutura para representar um livro
#[derive(Debug, Serialize, Deserialize)]
pub struct Livro {
    pub id: u32,
    pub titulo: String,
    pub autor: String,
    pub data_publicacao: NaiveDate,
    pub genero: Genero,
}

/*
pub fn validar_livro(livro: &Livro) -> Result<(), String> {
    if livro.titulo.trim().is_empty() {
        return Err("O título do livro não pode estar vazio.".to_string());
    }
    if livro.autor.trim().is_empty() {
        return Err("O autor do livro não pode estar vazio.".to_string());
    }
    if livro.data_publicacao > Utc::now().naive_local().date() {
        return Err("A data de publicação deve ser válida.".to_string());
    }
    Ok(())
} */