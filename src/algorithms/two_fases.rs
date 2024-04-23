
use crate::types::SimplexMethod;

impl SimplexMethod {
    
    // Primera fase del metodo de 2 fases. Se restan las filas 
    // correspondientes a las variables artificiales en la función objetivo

    fn first_fase(&mut self) {

        println!("Iniciando primera fase ...");

        // Restar filas con variable artificial (self.artificial_rows)
        // En la fila de la Función objetivo (self.increased[0])

        for a_index in self.artificial_rows.iter() {

            for i in 0..self.increased[0].len() {
                self.increased[0][i] = self.increased[0][i] - self.increased[*a_index][i]
            }
        }

        println!("\nFilas restadas en fila z...");

        self.update_table();
        self.print_table();

        println!("\nIniciando pivoteo...");

        self.pivoting();
    }

    // Segunda fase del metodo de 2 fases

    pub fn second_fase(&mut self) {

        println!("Iniciando segunda fase ...");

        // Inicialización de la forma aumentada en la segunda fase
        // Se eliminan las columnas correspondientes a las variables artificiales

        // Crear matríz sin variables artificiales
        // self.var_positions => diccionario con las posiciones de las variables

        let mut new_increased = Vec::new();

        for i in 0..self.increased.len() {

            let mut row = Vec::new();

            for j in 0..self.increased[0].len() {

                if !self.var_positions.get(&'a').unwrap().contains(&j) {

                    // Si la columna actual iterable corresponde a una
                    // variable artificial nos la saltamos

                    row.push(self.increased[i][j])
                }
            }

            new_increased.push(row);
        }

        // Actualizar las posiciones de las columnas de las variables a, e, h

        for (_key, value) in self.var_positions.iter_mut() {
            value.iter_mut().for_each(|x| *x -= self.artificial_rows.len()); 
        }

        // Añadir los coeficientes de la función objetivo en la nueva matríz

        for i in 1..self.n_vars + 1 {
            new_increased[0][i] = self.c[i].clone() * -1_f64;
        }
        
        // Reiniciar el pivot y actualizar la matríz

        self.pivot = (0, 0);
        self.increased = new_increased;

        self.init_sec_fase_table();
        self.update_table();
        self.print_table();

        self.pivoting();
    }

    pub fn two_fases(&mut self) {

        self.first_fase();
        self.second_fase();
    }
}


