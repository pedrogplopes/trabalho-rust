use crate::emprestimo_controller::EmprestimoController;
use crate::emprestimo::{Emprestimo, StatusEmprestimo};
use std::io;
use chrono::Utc;

pub fn gerenciar_emprestimos(emprestimo_controller: &mut EmprestimoController) {
    loop {
        println!("\n--- Gerenciar Empréstimos ---");
        println!("1. Adicionar Empréstimo");
        println!("2. Listar Empréstimos");
        println!("3. Alterar Status de Empréstimo");
        println!("4. Remover Empréstimo");
        println!("5. Voltar");
        println!("Escolha uma opção:");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler a entrada");
        let opcao = opcao.trim();

        match opcao {
            "1" => {
                let mut usuario = String::new();
                println!("Digite o nome do usuário:");
                io::stdin().read_line(&mut usuario).expect("Erro ao ler o nome do usuário");
                let usuario = usuario.trim();

                let mut id_livro = String::new();
                println!("Digite o ID do livro:");
                io::stdin().read_line(&mut id_livro).expect("Erro ao ler o ID do livro");
                let id_livro = id_livro.trim().parse::<u32>().unwrap_or(0);

                let emprestimo = Emprestimo {
                    id: emprestimo_controller.obter_qtd_emprestimos() as u32 + 1,
                    usuario: usuario.to_string(),
                    id_livro,
                    data_emprestimo: Utc::now().naive_local().date(),
                    status: StatusEmprestimo::Ativo,
                };

                if emprestimo_controller.adicionar_emprestimo(emprestimo).is_ok() {
                    println!("Empréstimo adicionado com sucesso!");
                } else {
                    println!("Erro ao adicionar o empréstimo!");
                }
            }
            "2" => emprestimo_controller.listar_emprestimos(),
            "3" => {
                let mut id = String::new();
                println!("Digite o ID do empréstimo para alterar o status:");
                io::stdin().read_line(&mut id).expect("Erro ao ler o ID");
                let id = id.trim().parse::<u32>().unwrap_or(0);
                if emprestimo_controller.alterar_status_emprestimo(id).is_ok() {
                    println!("Status do empréstimo alterado com sucesso!");
                } else {
                    println!("Erro ao alterar o status do empréstimo!");
                }
            }
            "4" => {
                let mut id = String::new();
                println!("Digite o ID do empréstimo a ser removido:");
                io::stdin().read_line(&mut id).expect("Erro ao ler o ID");
                let id = id.trim().parse::<u32>().unwrap_or(0);
                if emprestimo_controller.remover_emprestimo(id).is_ok() {
                    println!("Empréstimo removido com sucesso!");
                } else {
                    println!("Erro ao remover o empréstimo!");
                }
            }
            "5" => break,
            _ => println!("Opção inválida!"),
        }
    }
}

