mod livro;
mod livro_persistencia;
mod livro_controller;
mod emprestimo;
mod emprestimo_persistencia;
mod emprestimo_controller;
mod gerenciar_livros;
mod gerenciar_emprestimos;

use gerenciar_livros::gerenciar_livros;
use gerenciar_emprestimos::gerenciar_emprestimos;
use livro_controller::LivroController;
use emprestimo_controller::EmprestimoController;
use std::io;

fn main() {
    let mut livro_controller = LivroController::new();
    let mut emprestimo_controller = EmprestimoController::new();

    loop {
        println!("\n--- Sistema de Biblioteca ---");
        println!("1. Gerenciar Livros");
        println!("2. Gerenciar Empréstimos");
        println!("3. Sair");
        println!("Escolha uma opção:");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler a entrada");
        let opcao = opcao.trim();

        match opcao {
            "1" => gerenciar_livros(&mut livro_controller),
            "2" => gerenciar_emprestimos(&mut emprestimo_controller),
            "3" => {
                println!("Saindo do sistema...");
                break;
            }
            _ => println!("Opção inválida!"),
        }
    }
}
