
use crate::types::SimplexMethod;

impl SimplexMethod {
    
    // Caso de finalización especial en la segunda fase
    // Una parte del algoritmo debe finalizar cuando las 
    // variables de las restricciones (xn) sean 0

    pub fn should_finish_second_fase(&self) -> bool {

        for i in 1..self.n_vars + 1 {
            if self.increased[0][i] != 0f64 { return false }
        }

        true
    }

    // Inicialización de la forma aumentada en la segunda fase
    // Se eliminan las columnas correspondientes a las variables artificiales

    pub fn increased_to_second_fase(&mut self) {

        // Crear matríz sin variables artificiales
        // self.var_positions => diccionario con las posiciones de las variables

        let mut new_increased = Vec::new();

        for i in 0..self.increased.len() {

            let mut row = Vec::new();

            for j in 0..self.increased[0].len() {

                // Si la columna actual iterable corresponde a una
                // variable artificial nos la saltamos

                if !self.var_positions.get(&'a').unwrap().contains(&j) {
                    row.push(self.increased[i][j])
                }
            }

            new_increased.push(row);
        }

        // Se actualiza la fila de la función objetivo a los coheficientes originales. 
        // Manteniendo los valores de las variables de exceso y holgura.

        for i in 1..self.n_vars + 1 {
            new_increased[0][i] = self.c[i].clone()
        }

        self.increased = new_increased;
    }

    // Primera fase del metodo de 2 fases. Se restan las filas 
    // correspondientes a las variables artificiales en la función objetivo

    pub fn first_fase(&mut self) {

        println!("Iniciando primera fase ...");

        // self.artificial_rows corresponde a un vector con los indices
        // de las filas correspondientes a variables artificiales.

        for a_index in self.artificial_rows.iter() {

            for i in 0..self.increased[0].len() {
                self.increased[0][i] = self.increased[0][i] - self.increased[*a_index][i]
            }
        }

        println!("\nFilas artificiales restadas en fila z...");

        self.update_table();
        self.print_table();

        println!("\nIniciando pivoteo...");

        while !self.should_finish() {
            self.pivoting();
        }
    }

    // Segunda fase del metodo de 2 fases
    pub fn second_fase(&mut self) {

        println!("Iniciando segunda fase ...");

        // Se reestablece el pivot del algoritmo en (0, 0)
        // y se establece la fase del algoritmo en 2

        self.pivot = (0, 0);
        self.fase = 2;

        // Inicialización de la forma aumentada de segunda fase

        self.increased_to_second_fase();
        self.init_sec_fase_table();

        self.update_table();
        self.print_table();

        // Pivotear mientras las variables de las restricciones 
        // del problema sean distintas de 0

        while !self.should_finish_second_fase() {
            self.pivoting();
        }

        // Iniciar segundo pivoteo en caso de que queden números negativos
        // o que haya pivotes validos en la forma aumentada

        while !self.should_finish() {
            self.pivoting();
        }
    }

    pub fn two_fases(&mut self) {

        self.first_fase();
        self.second_fase();
    }
}


