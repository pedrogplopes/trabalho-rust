use crate::livro::Livro;
use crate::livro_persistencia::{carregar_livros, salvar_livros};

pub struct LivroController {
    pub livros: Vec<Livro>,
}
impl LivroController {
    pub fn new() -> Self {
        Self {
            livros: carregar_livros(),
        }
    }

    pub fn criar_livro(&mut self, livro: Livro) -> Result<(), String> {
        self.livros.push(livro);
        salvar_livros(&self.livros).map_err(|e| e.to_string())
    }

    pub fn remover_livro(&mut self, id: u32) -> Result<(), String> {
        if let Some(pos) = self.livros.iter().position(|l| l.id == id) {
            self.livros.remove(pos);
            salvar_livros(&self.livros).map_err(|e| e.to_string())
        } else {
            Err("Livro não encontrado".to_string())
        }
    }

    pub fn listar_livros(&self) {
        for livro in &self.livros {
            println!("{:?}", livro);
        }
    }
}