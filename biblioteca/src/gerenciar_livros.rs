use crate::livro_controller::LivroController;
use crate::livro::Livro;
use crate::livro::Genero;
use std::io;
use chrono::{NaiveDate, Utc};

pub fn gerenciar_livros(livro_controller: &mut LivroController) {
    loop {
        println!("\n--- Gerenciar Livros ---");
        println!("1. Adicionar Livro");
        println!("2. Listar Livros");
        println!("3. Atualizar Livro");
        println!("4. Remover Livro");
        println!("5. Voltar");
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

                let genero = selecionar_genero();

                let data_publicacao = inserir_data_publicacao();

                let livro = Livro {
                    id: livro_controller.livros.len() as u32 + 1,
                    genero,
                    titulo: titulo.to_string(),
                    autor: autor.to_string(),
                    data_publicacao,
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
                println!("Digite o ID do livro a ser alterado:");
                io::stdin().read_line(&mut id).expect("Erro ao ler o ID");
                let id = id.trim().parse::<u32>().unwrap_or(0);

                if let Some(livro) = livro_controller.obter_livro(id) {
                     println!("Livro encontrado: {:?}", livro);

                    let mut novo_titulo = String::new();
                    println!("Digite o novo título (ou pressione Enter para manter '{}'):", livro.titulo);
                    io::stdin().read_line(&mut novo_titulo).expect("Erro ao ler título");
                    let novo_titulo = novo_titulo.trim();
                    let titulo_atualizado = if novo_titulo.is_empty() {
                        livro.titulo.clone()
                    } else {
                        novo_titulo.to_string()
                    };

                    let mut novo_autor = String::new();
                    println!("Digite o novo autor (ou pressione Enter para manter '{}'):", livro.autor);
                    io::stdin().read_line(&mut novo_autor).expect("Erro ao ler autor");
                    let novo_autor = novo_autor.trim();
                    let autor_atualizado = if novo_autor.is_empty() {
                        livro.autor.clone()
                    } else {
                        novo_autor.to_string()
                    };

                    let genero_atualizado = selecionar_genero();

                    let data_publicacao_atualizada = inserir_data_publicacao();

                    
                    let livro_atualizado = Livro {
                        id: livro.id,
                        titulo: titulo_atualizado,
                        autor: autor_atualizado,
                        genero: genero_atualizado,
                        data_publicacao: data_publicacao_atualizada,
                    };

                    if livro_controller.atualizar_livro(id, livro_atualizado).is_ok() {
                        println!("Livro atualizado com sucesso!");
                    } else {
                        println!("Erro ao atualizar o livro!");
                    }
                } else {
                    println!("Livro com ID {} não encontrado!", id);
                }
}
            "4" => {
                let mut id = String::new();
                println!("Digite o ID do livro a ser removido:");
                io::stdin().read_line(&mut id).expect("Erro ao ler o ID");
                let id = id.trim().parse::<u32>().unwrap_or(0);
                if livro_controller.remover_livro(id).is_ok() {
                    println!("Livro removido com sucesso!");
                } else {
                    println!("Livro com ID {} não encontrado!", id);
                }
            }
            "5" => break,
            _ => println!("Opção inválida!"),
        }
    }
}


fn selecionar_genero() -> Genero {
    println!("Selecione o gênero do livro:");
    println!("1. Nenhum");
    println!("2. Ficção");
    println!("3. Não Ficção");
    println!("4. Fantasia");
    println!("5. Ciência");
    println!("6. Romance");
    let mut opcao = String::new();
    io::stdin().read_line(&mut opcao).expect("Erro ao ler o gênero");
    match opcao.trim() {
        "1" => Genero::None,
        "2" => Genero::Ficcao,
        "3" => Genero::NaoFiccao,
        "4" => Genero::Fantasia,
        "5" => Genero::Ciencia,
        "6" => Genero::Romance,
        _ => {
            println!("Opção inválida! Usando gênero padrão: Nenhum.");
            Genero::None
        }
    }
}


fn inserir_data_publicacao() -> NaiveDate {
    println!("Digite a data de publicação (formato: YYYY-MM-DD):");
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Erro ao ler a data");
    NaiveDate::parse_from_str(data.trim(), "%Y-%m-%d").unwrap_or_else(|_| {
        println!("Formato inválido! Usando a data atual como padrão.");
        Utc::now().naive_local().date()
    })
}


