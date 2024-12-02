#[cfg(test)]
mod tests {
    use biblioteca::emprestimo_controller::EmprestimoController;
    use biblioteca::emprestimo::{Emprestimo, StatusEmprestimo};
    use chrono::NaiveDate;

    fn criar_controller_com_dados() -> EmprestimoController {
        let mut controller = EmprestimoController::new();
        controller
            .adicionar_emprestimo(Emprestimo {
                id: 1,
                id_livro: 1,
                usuario: "João".to_string(),
                data_emprestimo: NaiveDate::from_ymd_opt(2023, 12, 1)
                    .expect("Data inválida fornecida!"),
                status: StatusEmprestimo::Ativo,
            })
            .unwrap();
        controller
    }

    #[test]
    fn test_adicionar_emprestimo() {
        let mut controller = EmprestimoController::new();

        let emprestimo = Emprestimo {
            id: 1,
            id_livro: 1,
            usuario: "Maria".to_string(),
            data_emprestimo: NaiveDate::from_ymd_opt(2023, 12, 2)
                .expect("Data inválida fornecida!"),
            status: StatusEmprestimo::Ativo,
        };

        assert!(controller.adicionar_emprestimo(emprestimo).is_ok());
        assert_eq!(controller.obter_emprestimos().len(), 1);
    }

    #[test]
    fn test_remover_emprestimo() {
        let mut controller = criar_controller_com_dados();

        assert!(controller.remover_emprestimo(1).is_ok());
        assert_eq!(controller.obter_emprestimos().len(), 0);
    }

    #[test]
    fn test_remover_emprestimo_inexistente() {
        let mut controller = criar_controller_com_dados();

        let resultado = controller.remover_emprestimo(999);
        assert!(resultado.is_err(), "Empréstimo inexistente não deveria ser removido");
    }


    #[test]
    fn test_alterar_status_emprestimo_inexistente() {
        let mut controller = criar_controller_com_dados();

        let resultado = controller.alterar_status_emprestimo(999);
        assert!(resultado.is_err(), "Não deveria ser possível alterar o status de empréstimo inexistente");
    }

    #[test]
    fn test_listar_emprestimos() {
        let controller = criar_controller_com_dados();

        assert_eq!(controller.obter_emprestimos().len(), 1);
    }
}
