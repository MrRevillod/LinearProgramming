
use crate::types::{ProblemKind, SimplexMethod};
use crate::algorithms::table::print_matrix;

impl SimplexMethod {
    
    // Primera fase del metodo de 2 fases. Se restan las filas 
    // correspondientes a las variables artificiales en la función objetivo

    fn first_fase(&mut self) {

        // Iniciar los coeficientes de la función objetivo 0

        for i in 1..self.n_vars + 1 {
            self.increased[0][i] = 0_f64;
        }

        print_matrix("\nIniciando primera fase ...", &self.increased, &self.table);

        // Restar filas con variable artificial (self.artificial_rows)
        // En la fila de la Función objetivo (self.increased[0])

        for a_indexes in self.artificials_variables.iter() {
            for j in 1..self.increased[0].len() {
                self.increased[0][j] += self.increased[a_indexes.0][j] * -1_f64;
            }
        }

        print_matrix("\nFilas artificiales restadas en z ...", &self.increased, &self.table);

        self.pivoting();
    }

    pub fn second_fase(&mut self) {

        println!("\nIniciando segunda fase ...");

        let mut new_increased = Vec::new();

        for i in 0..self.increased.len() {

            let mut row = Vec::new();

            for j in 0..self.increased[0].len() {

                if !self.artificials_variables.iter().any(|(_r, c)| *c == j) {
                    row.push(self.increased[i][j]);
                    continue;
                }

                self.table[0][j + 1] = String::from("");
            }

            new_increased.push(row);
        }

        self.table[0] = self.table[0].iter().filter(|&x| !x.is_empty()).cloned().collect();

        new_increased[0][0] = 1_f64;

        print_matrix("Despues de eliminar columnas artificiales ...", &new_increased, &self.table);

        for i in 1..self.n_vars + 1 {

            new_increased[0][i] = match self.kind {
                ProblemKind::Maximize => -self.c[i],
                ProblemKind::Minimize => self.c[i]
            }
        }

        print_matrix("\nDespues reemplazar variables de la z ...", &new_increased, &self.table);

        for vb in self.basic_vars.iter_mut() {
            if vb.1 > self.n_vars + 1 { vb.1 -= 1 }
        }

        for vb in self.basic_vars.iter() {

            let value = new_increased[0][vb.1];

            for j in 1..new_increased[0].len() {
                new_increased[0][j] -= value * new_increased[vb.0][j];
            }
        }

        print_matrix("\nDespues de aplicar eliminación gaussiana en variables básicas ...", &new_increased, &self.table);
        
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

