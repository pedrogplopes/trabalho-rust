use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use crate::livro::Livro;

// Enum para o status do empréstimo
#[derive(Debug, Serialize, Deserialize)]
pub enum StatusEmprestimo {
    Ativo,
    Finalizado,
}

impl PartialEq<&str> for StatusEmprestimo {
    fn eq(&self, other: &&str) -> bool {
        match self {
            StatusEmprestimo::Ativo => *other == "ativo",
            StatusEmprestimo::Finalizado => *other == "finalizado",
            // adicione mais comparações se houver mais variantes
        }
    }
}

// Estrutura para representar um empréstimo
#[derive(Debug, Serialize, Deserialize)]
pub struct Emprestimo {
    pub id: u32,
    pub id_livro: u32,
    pub usuario: String,
    pub data_emprestimo: NaiveDate,
    pub status: StatusEmprestimo,
}

// Função para validar um empréstimo
pub fn validar_emprestimo(emprestimo: &Emprestimo) -> Result<(), String> {
    if emprestimo.id == 0 {
        return Err("O ID do empréstimo deve ser um número positivo.".to_string());
    }
    if emprestimo.usuario.trim().is_empty() {
        return Err("O nome do usuário não pode estar vazio.".to_string());
    }
    if emprestimo.data_emprestimo > Utc::now().naive_local().date() {
        return Err("A data de empréstimo precisa ser válida.".to_string());
    }
    Ok(())
}