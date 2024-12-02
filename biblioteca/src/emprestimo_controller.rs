use std::io::{self, Write};
use std::fs::File;
use crate::emprestimo::{Emprestimo, StatusEmprestimo};
use crate::emprestimo_persistencia::{carregar_emprestimos, salvar_emprestimos};

pub struct EmprestimoController {
    emprestimos: Vec<Emprestimo>,
}

impl EmprestimoController {
    pub fn new() -> Self {
        Self {
            emprestimos: carregar_emprestimos(),
        }
    }

    pub fn adicionar_emprestimo(&mut self, emprestimo: Emprestimo) -> Result<(), String> {
        self.emprestimos.push(emprestimo);
        salvar_emprestimos(&self.emprestimos).map_err(|e| e.to_string())
    }

    pub fn remover_emprestimo(&mut self, id: u32) -> Result<(), String> {
        if let Some(pos) = self.emprestimos.iter().position(|e| e.id == id) {
            self.emprestimos.remove(pos);
            salvar_emprestimos(&self.emprestimos).map_err(|e| e.to_string())
        } else {
            Err("Empréstimo não encontrado".to_string())
        }
    }

    pub fn listar_emprestimos(&self) {
        if self.emprestimos.is_empty() {
            println!("Nenhum empréstimo registrado.");
        } else {
            for emprestimo in &self.emprestimos {
                println!("{:?}", emprestimo);
            }
        }
    }
    
    #[allow(dead_code)] 
    pub fn obter_emprestimos(&self) -> &Vec<Emprestimo> {
        &self.emprestimos
    }

    pub fn obter_qtd_emprestimos(&self) -> usize {
            self.emprestimos.len()
        }
        

    pub fn alterar_status_emprestimo(&mut self, id: u32) -> Result<(), String> {
        if let Some(emprestimo) = self.emprestimos.iter_mut().find(|e| e.id == id) {
            emprestimo.status = if emprestimo.status == "ativo" {
                StatusEmprestimo::Finalizado
            } else {
                StatusEmprestimo::Ativo
            };
            match self.salvar_emprestimos() {
                Ok(_) => println!("Empréstimos salvos com sucesso."),
                Err(e) => eprintln!("Erro ao salvar os empréstimos: {}", e),
            }
            Ok(())
        } else {
            Err(format!("Empréstimo com ID {} não encontrado.", id))
        }
    }

    pub fn salvar_emprestimos(&self) -> Result<(), io::Error> {
        let json_data = serde_json::to_string(&self.emprestimos)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let mut file = File::create("emprestimos.json")?;

        file.write_all(json_data.as_bytes())?;

        Ok(())
    }

    pub fn obter_emprestimo_mut(&mut self, id: u32) -> Option<&mut Emprestimo> {
        self.emprestimos.iter_mut().find(|e| e.id == id)
    }
}
