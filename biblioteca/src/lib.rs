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
        pub publicado: String,
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

        #[ink(message)]
        pub fn adicionar_livro(&mut self, titulo: String, autor: String, publicado: String) -> Result<u32, String> {
            if titulo.is_empty() || autor.is_empty() || publicado.is_empty() {
                return Err("Título, autor e data de publicação não podem estar vazios".into());
            }

            let id = self.next_livro_id;
            let livro = Livro {
                id,
                titulo,
                autor,
                disponivel: true,
                publicado,
            };

            self.livros.insert(id, &livro);
            self.next_livro_id = self.next_livro_id.checked_add(1).ok_or("ID overflow")?;

            Ok(id)
        }

        #[ink(message)]
        pub fn remover_livro(&mut self, livro_id: u32) -> Result<(), String> {
            if self.livros.get(livro_id).is_none() {
                return Err("Livro não encontrado".into());
            }
            self.livros.remove(livro_id);
            Ok(())
        }

        #[ink(message)]
        pub fn editar_livro(&mut self, livro_id: u32, novo_titulo: String, novo_autor: String, nova_data: String) -> Result<(), String> {
            let mut livro = self.livros.get(livro_id).ok_or("Livro não encontrado")?;
            livro.titulo = novo_titulo;
            livro.autor = novo_autor;
            livro.publicado = nova_data;
            self.livros.insert(livro_id, &livro);
            Ok(())
        }

        #[ink(message)]
        pub fn buscar_livro_por_titulo(&self, titulo: String) -> Vec<Livro> {
            (0..self.next_livro_id)
                .filter_map(|id| self.livros.get(id))
                .filter(|livro| livro.titulo.contains(&titulo))
                .collect()
        }

        #[ink(message)]
        pub fn buscar_livro_por_autor(&self, autor: String) -> Vec<Livro> {
            (0..self.next_livro_id)
                .filter_map(|id| self.livros.get(id))
                .filter(|livro| livro.autor == autor)
                .collect()
        }

        #[ink(message)]
        pub fn editar_emprestimo(&mut self, emprestimo_id: u32, novo_usuario: String, nova_data_emprestimo: String) -> Result<(), String> {
            let mut emprestimo = self.emprestimos.get(emprestimo_id).ok_or("Empréstimo não encontrado")?;
            emprestimo.usuario = novo_usuario;
            emprestimo.data_emprestimo = nova_data_emprestimo;
            self.emprestimos.insert(emprestimo_id, &emprestimo);
            Ok(())
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
            (0..self.next_livro_id).filter_map(|id| self.livros.get(id)).collect()
        }

        #[ink(message)]
        pub fn listar_emprestimos(&self) -> Vec<Emprestimo> {
            (0..self.next_emprestimo_id).filter_map(|id| self.emprestimos.get(id)).collect()
        }
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
