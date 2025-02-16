#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::string::String;
use ink::storage::Mapping;

#[ink::contract]
mod biblioteca_ink {
    use super::*;

    #[ink(storage)]
    pub struct Biblioteca {
        livros: Mapping<u32, Livro>,
        proximo_id_livro: u32,
        emprestimos: Mapping<u32, Emprestimo>,
        proximo_id_emprestimo: u32,
        livros_emprestados: Mapping<u32, u32>,
    }

    impl Default for Biblioteca {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Biblioteca {
        #[ink(constructor, selector = 0x0001)]
        pub fn new() -> Self {
            Self {
                livros: Mapping::new(),
                proximo_id_livro: 0,
                emprestimos: Mapping::new(),
                proximo_id_emprestimo: 0,
                livros_emprestados: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn criar_livro(
            &mut self,
            titulo: String,
            autor: String,
            data_publicacao: u64,
            genero: Genero,
        ) -> Result<u32, String> {
            if titulo.trim().is_empty() || autor.trim().is_empty() {
                return Err("Título e autor não podem ser vazios".into());
            }
            let id = self.proximo_id_livro;
            let livro = Livro {
                id,
                titulo,
                autor,
                data_publicacao,
                genero,
            };
            self.livros.insert(id, &livro);
            self.proximo_id_livro = self.proximo_id_livro.saturating_add(1);
            Ok(id)
        }

        #[ink(message)]
        pub fn obter_livro(&self, id: u32) -> Option<Livro> {
            self.livros.get(id)
        }

        #[ink(message)]
        pub fn emprestar_livro(
            &mut self,
            id_livro: u32,
            usuario: AccountId,
        ) -> Result<(), String> {
            if self.livros_emprestados.get(id_livro).is_some() {
                return Err("Livro já emprestado".into());
            }
            let _livro = self.livros.get(id_livro).ok_or("Livro não encontrado")?;
            let now = self.env().block_timestamp();
            let emprestimo = Emprestimo {
                id: self.proximo_id_emprestimo,
                id_livro,
                usuario,
                data_emprestimo: now,
                data_devolucao: None,
                status: StatusEmprestimo::Ativo,
            };
            self.emprestimos.insert(self.proximo_id_emprestimo, &emprestimo);
            self.livros_emprestados.insert(id_livro, &self.proximo_id_emprestimo);
            self.proximo_id_emprestimo = self.proximo_id_emprestimo.saturating_add(1);
            self.env().emit_event(EmprestimoRealizado {
                id_emprestimo: emprestimo.id,
                id_livro,
                usuario,
                data_emprestimo: now,
            });
            Ok(())
        }

        #[ink(message)]
        pub fn devolver_livro(&mut self, id_emprestimo: u32) -> Result<(), String> {
            let mut emprestimo = self.emprestimos.get(id_emprestimo)
                .ok_or("Empréstimo não encontrado")?;
            if emprestimo.status == StatusEmprestimo::Finalizado {
                return Err("Empréstimo já finalizado".into());
            }
            emprestimo.status = StatusEmprestimo::Finalizado;
            emprestimo.data_devolucao = Some(self.env().block_timestamp());
            self.emprestimos.insert(id_emprestimo, &emprestimo);
            self.livros_emprestados.remove(emprestimo.id_livro);
            self.env().emit_event(EmprestimoFinalizado {
                id_emprestimo,
                data_devolucao: emprestimo.data_devolucao.unwrap(),
            });
            Ok(())
        }
    }

    #[repr(u8)]
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum Genero {
        None = 0,
        Ficcao = 1,
        NaoFiccao = 2,
        Fantasia = 3,
        Ciencia = 4,
        Romance = 5,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Livro {
        id: u32,
        titulo: String,
        autor: String,
        data_publicacao: u64,
        genero: Genero,
    }

    #[repr(u8)]
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum StatusEmprestimo {
        Ativo = 0,
        Finalizado = 1,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Emprestimo {
        id: u32,
        id_livro: u32,
        usuario: AccountId,
        data_emprestimo: u64,
        data_devolucao: Option<u64>,
        status: StatusEmprestimo,
    }

    #[ink(event)]
    pub struct EmprestimoRealizado {
        #[ink(topic)]
        id_emprestimo: u32,
        #[ink(topic)]
        id_livro: u32,
        #[ink(topic)]
        usuario: AccountId,
        data_emprestimo: u64,
    }

    #[ink(event)]
    pub struct EmprestimoFinalizado {
        #[ink(topic)]
        id_emprestimo: u32,
        data_devolucao: u64,
    }
}