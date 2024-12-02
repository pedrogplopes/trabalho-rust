use crate::emprestimo_controller::EmprestimoController;
use crate::emprestimo::{Emprestimo, StatusEmprestimo};
use std::io;
use chrono::{NaiveDate, Utc};

pub fn gerenciar_emprestimos(emprestimo_controller: &mut EmprestimoController) {
    loop {
        println!("\n--- Gerenciar Empréstimos ---");
        println!("1. Adicionar Empréstimo");
        println!("2. Listar Empréstimos");
        println!("3. Alterar dados do Empréstimo");
        println!("4. Alterar Status de Empréstimo");
        println!("5. Remover Empréstimo");
        println!("6. Voltar");
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
                
                let id_livro: u32 = match id_livro.trim().parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Entrada inválida para ID do livro. O ID será definido como 0.");
                        0
                    }
                };

                let data_emprestimo = inserir_data_emprestimo();

                let emprestimo = Emprestimo {
                    id: emprestimo_controller.obter_qtd_emprestimos() as u32 + 1,
                    usuario: usuario.to_string(),
                    id_livro,
                    data_emprestimo,
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
                // Alterar empréstimo (ID do livro, nome do autor e data)
                let mut id = String::new();
                println!("Digite o ID do empréstimo para alterar:");
                io::stdin().read_line(&mut id).expect("Erro ao ler o ID");
                let id: u32 = id.trim().parse().unwrap_or(0);

                if let Some(emprestimo) = emprestimo_controller.obter_emprestimo_mut(id) {
                    let mut novo_id_livro = String::new();
                    println!("Digite o novo ID do livro (ou pressione Enter para manter o atual):");
                    io::stdin().read_line(&mut novo_id_livro).expect("Erro ao ler o novo ID do livro");
                    if let Ok(id_livro) = novo_id_livro.trim().parse() {
                        emprestimo.id_livro = id_livro;
                    } else if !novo_id_livro.trim().is_empty() {
                        println!("ID do livro inválido. O ID já registrado será mantido.");
                    }

                    let mut novo_autor = String::new();
                    println!("Digite o novo nome do autor (ou pressione Enter para manter o atual):");
                    io::stdin().read_line(&mut novo_autor).expect("Erro ao ler o novo nome do autor");
                    if !novo_autor.trim().is_empty() {
                        emprestimo.usuario = novo_autor.trim().to_string();
                    }

                    let mut nova_data = String::new();
                    println!("Digite a nova data de empréstimo (formato: YYYY-MM-DD) ou pressione Enter para manter a atual:");
                    io::stdin().read_line(&mut nova_data).expect("Erro ao ler a nova data");
                    if let Ok(data) = NaiveDate::parse_from_str(nova_data.trim(), "%Y-%m-%d") {
                        emprestimo.data_emprestimo = data;
                    } else if !nova_data.trim().is_empty() {
                        println!("Data inválida. A data já registrada será mantida.");
                    }

                    if emprestimo_controller.salvar_emprestimos().is_ok() {
                        println!("Empréstimo alterado com sucesso!");
                    } else {
                        println!("Erro ao alterar o empréstimo!");
                    }
                } else {
                    println!("Empréstimo não encontrado.");
                }
            }
            "4" => {
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
            "5" => {
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
            "6" => break,
            _ => println!("Opção inválida!"),
        }
    }
}

/// Função para inserir a data de empréstimo
fn inserir_data_emprestimo() -> NaiveDate {
    println!("Digite a data do empréstimo (formato: YYYY-MM-DD):");
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Erro ao ler a data");
    NaiveDate::parse_from_str(data.trim(), "%Y-%m-%d").unwrap_or_else(|_| {
        println!("Formato inválido! Usando a data atual como padrão.");
        Utc::now().naive_local().date()
    })
}

