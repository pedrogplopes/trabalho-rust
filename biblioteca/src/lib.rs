#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod sistema_emprestimo {
    use ink::prelude::{string::String, vec::Vec};
    use ink::storage::Mapping;

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Livro {
        pub id: u32,
        pub titulo: String,
        pub autor: String,
        pub disponivel: bool,
    }

    #[derive(scale::Encode, scale::Decode, Clone, Debug, PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Emprestimo {
        pub livro_id: u32,
        pub usuario: String,
        pub data_emprestimo: String,
        pub data_devolucao: Option<String>,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct EmprestimoManager {
        livros: Mapping<u32, Livro>,
        emprestimos: Mapping<u32, Emprestimo>,
        next_livro_id: u32,
        next_emprestimo_id: u32,
    }

    impl EmprestimoManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        // ----- CRUD para Livros -----
        #[ink(message)]
        pub fn adicionar_livro(&mut self, titulo: String, autor: String) -> Result<u32, String> {
            if titulo.is_empty() || autor.is_empty() {
                return Err("Título e autor não podem estar vazios".into());
            }

            let id = self.next_livro_id;
            let livro = Livro {
                id,
                titulo,
                autor,
                disponivel: true,
            };

            self.livros.insert(id, &livro);
            self.next_livro_id = self.next_livro_id.checked_add(1).ok_or("ID overflow")?;

            Ok(id)
        }

        #[ink(message)]
        pub fn emprestar_livro(&mut self, livro_id: u32, usuario: String, data_emprestimo: String) -> Result<u32, String> {
            let mut livro = self.livros.get(livro_id).ok_or("Livro não encontrado")?;
            if !livro.disponivel {
                return Err("Livro não está disponível".into());
            }

            let emprestimo_id = self.next_emprestimo_id;
            let emprestimo = Emprestimo {
                livro_id,
                usuario,
                data_emprestimo,
                data_devolucao: None,
            };

            livro.disponivel = false;
            self.livros.insert(livro_id, &livro);
            self.emprestimos.insert(emprestimo_id, &emprestimo);
            self.next_emprestimo_id = self.next_emprestimo_id.checked_add(1).ok_or("ID overflow")?;

            Ok(emprestimo_id)
        }

        #[ink(message)]
        pub fn devolver_livro(&mut self, emprestimo_id: u32, data_devolucao: String) -> Result<(), String> {
            let mut emprestimo = self.emprestimos.get(emprestimo_id).ok_or("Empréstimo não encontrado")?;
            let mut livro = self.livros.get(emprestimo.livro_id).ok_or("Livro não encontrado")?;

            livro.disponivel = true;
            emprestimo.data_devolucao = Some(data_devolucao);

            self.livros.insert(emprestimo.livro_id, &livro);
            self.emprestimos.insert(emprestimo_id, &emprestimo);
            Ok(())
        }

        #[ink(message)]
        pub fn listar_livros(&self) -> Vec<Livro> {
            let mut lista = Vec::new();
            for id in 0..self.next_livro_id {
                if let Some(livro) = self.livros.get(id) {
                    lista.push(livro);
                }
            }
            lista
        }

        #[ink(message)]
        pub fn listar_emprestimos(&self) -> Vec<Emprestimo> {
            let mut lista = Vec::new();
            for id in 0..self.next_emprestimo_id {
                if let Some(emprestimo) = self.emprestimos.get(id) {
                    lista.push(emprestimo);
                }
            }
            lista
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_adicionar_livro() {
            let mut manager = EmprestimoManager::new();
            let result = manager.adicionar_livro("Rust Book".into(), "Steve Klabnik".into());
            assert!(result.is_ok());
        }

        #[ink::test]
        fn test_emprestar_livro() {
            let mut manager = EmprestimoManager::new();
            let livro_id = manager.adicionar_livro("Rust Book".into(), "Steve Klabnik".into()).unwrap();
            let result = manager.emprestar_livro(livro_id, "João".into(), "10-08-2024".into());
            assert!(result.is_ok());
        }

        #[ink::test]
        fn test_devolver_livro() {
            let mut manager = EmprestimoManager::new();
            let livro_id = manager.adicionar_livro("Rust Book".into(), "Steve Klabnik".into()).unwrap();
            let emprestimo_id = manager.emprestar_livro(livro_id, "João".into(), "10-08-2024".into()).unwrap();
            let result = manager.devolver_livro(emprestimo_id, "15-08-2024".into());
            assert!(result.is_ok());
        }
    }
}
