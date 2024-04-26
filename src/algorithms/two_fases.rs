
use crate::types::{ProblemKind, SimplexMethod};

impl SimplexMethod {
    
    // Primera fase del metodo de 2 fases. Se restan las filas 
    // correspondientes a las variables artificiales en la función objetivo

    fn first_fase(&mut self) {

        println!("Iniciando primera fase ...");

        // Iniciar los coeficientes de la función objetivo 0

        for i in 1..self.n_vars + 1 {
            self.increased[0][i] = 0_f64;
        }

        self.print_increased();

        println!("Filas artificiales restadas en z ...");

        // Restar filas con variable artificial (self.artificial_rows)
        // En la fila de la Función objetivo (self.increased[0])

        for a_indexes in self.artificials_variables.iter() {
            for j in 1..self.increased[0].len() {
                self.increased[0][j] += self.increased[a_indexes.0][j] * -1_f64;
            }
        }

        self.print_increased();
        self.pivoting();
    }

    pub fn second_fase(&mut self) {

        println!("Iniciando segunda fase ...");

        let mut new_increased = Vec::new();

        for i in 0..self.increased.len() {

            let mut row = Vec::new();

            for j in 0..self.increased[0].len() {

                if !self.artificials_variables.iter().any(|(_r, c)| *c == j) {
                    row.push(self.increased[i][j])
                }
            }

            new_increased.push(row);
        }

        new_increased[0][0] = 1_f64;

        print_matrix(&new_increased, "Despues de eliminar columnas artificiales\n");

        for i in 1..self.n_vars + 1 {

            new_increased[0][i] = match self.kind {
                ProblemKind::Maximize => -self.c[i],
                ProblemKind::Minimize => self.c[i]
            }
        }

        print_matrix(&new_increased, "Despues de reemplazar variables de la z\n");

        for vb in self.basic_vars.iter_mut() {

            if vb.1 > self.n_vars + 1 {
                vb.1 -= 1
            }
        }

        for vb in self.basic_vars.iter() {

            let value = new_increased[0][vb.1];

            for j in 1..new_increased[0].len() {
                new_increased[0][j] -= value * new_increased[vb.0][j];
            }
        }
        
        print_matrix(&new_increased, "Despues de aplicar eliminación gaussiana en variables básicas ...\n");
        
        self.pivot = (0, 0);
        self.increased = new_increased;
        self.basic_vars.clear();

        self.pivoting();
    }

    pub fn two_fases(&mut self) {

        self.first_fase();
        self.second_fase();
    }
}

pub fn print_matrix(matrix: &Vec<Vec<f64>>, text: &str) {

    println!("{}", text);

    for row in matrix {

        for item in row {
            print!("{:<10}", format!("{:.4}", item));
        }

        println!();
    }

    println!();
}
