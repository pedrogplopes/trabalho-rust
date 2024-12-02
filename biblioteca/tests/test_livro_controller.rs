#[cfg(test)]
mod tests {
    use biblioteca::livro_controller::LivroController;
    use biblioteca::livro::{Livro, Genero};
    use chrono::NaiveDate;

    fn criar_controller_com_dados() -> LivroController {
        let mut controller = LivroController::new();
        controller
            .criar_livro(Livro {
                id: 1,
                titulo: "O Hobbit".to_string(),
                autor: "J.R.R. Tolkien".to_string(),
                genero: Genero::Ficcao,
                data_publicacao: NaiveDate::from_ymd_opt(1937, 9, 21)
                    .expect("Data inválida fornecida!"),
            })
            .unwrap();
        controller
    }

    #[test]
    fn test_adicionar_livro() {
        let mut controller = LivroController::new();

        let livro = Livro {
            id: 1,
            titulo: "1984".to_string(),
            autor: "George Orwell".to_string(),
            genero: Genero::Ficcao,
            data_publicacao: NaiveDate::from_ymd_opt(1949, 6, 8)
                .expect("Data inválida fornecida!"),
        };

        assert!(controller.criar_livro(livro).is_ok());
        assert_eq!(controller.livros.len(), 1);
        assert_eq!(controller.livros[0].titulo, "1984");
    }

    #[test]
    fn test_atualizar_livro() {
        let mut controller = criar_controller_com_dados();

        let livro_atualizado = Livro {
            id: 1,
            titulo: "O Senhor dos Anéis".to_string(),
            autor: "J.R.R. Tolkien".to_string(),
            genero: Genero::Fantasia,
            data_publicacao: NaiveDate::from_ymd_opt(1954, 7, 29)
                .expect("Data inválida fornecida!"),
        };

        assert!(controller.atualizar_livro(1, livro_atualizado).is_ok());
        assert_eq!(controller.livros[0].titulo, "O Senhor dos Anéis");
    }

    #[test]
    fn test_titulo_vazio() {
        let mut controller = LivroController::new();

        let livro = Livro {
            id: 1,
            titulo: "".to_string(),
            autor: "Autor Teste".to_string(),
            genero: Genero::Ficcao,
            data_publicacao: NaiveDate::from_ymd_opt(2020, 1, 1)
                .expect("Data inválida fornecida!"),
        };

        let resultado = controller.criar_livro(livro);
        assert!(resultado.is_err(), "O livro com título vazio não deveria ser adicionado");
    }

    #[test]
    fn test_autor_vazio() {
        let mut controller = LivroController::new();

        let livro = Livro {
            id: 1,
            titulo: "Título de Teste".to_string(),
            autor: "".to_string(),
            genero: Genero::Ficcao,
            data_publicacao: NaiveDate::from_ymd_opt(2020, 1, 1)
                .expect("Data inválida fornecida!"),
        };

        let resultado = controller.criar_livro(livro);
        assert!(resultado.is_err(), "O livro com autor vazio não deveria ser adicionado");
    }

}
