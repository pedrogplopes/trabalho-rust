use crate::livro_controller::LivroController;
use crate::livro::Livro;
use crate::livro::Genero;
use std::io;
use chrono::Utc;

pub fn gerenciar_livros(livro_controller: &mut LivroController) {
    loop {
        println!("\n--- Gerenciar Livros ---");
        println!("1. Adicionar Livro");
        println!("2. Listar Livros");
        println!("3. Remover Livro");
        println!("4. Voltar");
        println!("Escolha uma opção:");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler a entrada");
        let opcao = opcao.trim();

        match opcao {
            "1" => {
                let mut titulo = String::new();
                println!("Digite o título do livro:");
                io::stdin().read_line(&mut titulo).expect("Erro ao ler título");
                let titulo = titulo.trim();

                let mut autor = String::new();
                println!("Digite o autor do livro:");
                io::stdin().read_line(&mut autor).expect("Erro ao ler autor");
                let autor = autor.trim();

                let livro = Livro {
                    id: livro_controller.livros.len() as u32 + 1,
                    genero: Genero::None,
                    titulo: titulo.to_string(),
                    autor: autor.to_string(),
                    data_publicacao: Utc::now().naive_local().date(),
                };

                if livro_controller.criar_livro(livro).is_ok() {
                    println!("Livro adicionado com sucesso!");
                } else {
                    println!("Erro ao adicionar o livro!");
                }
            }
            "2" => livro_controller.listar_livros(),
            "3" => {
                let mut id = String::new();
                println!("Digite o ID do livro a ser removido:");
                io::stdin().read_line(&mut id).expect("Erro ao ler o ID");
                let id = id.trim().parse::<u32>().unwrap_or(0);
                if livro_controller.remover_livro(id).is_ok() {
                    println!("Livro removido com sucesso!");
                } else {
                    println!("Erro ao remover o livro!");
                }
            }
            "4" => break,
            _ => println!("Opção inválida!"),
        }
    }
}


