
#![allow(dead_code)]

use crate::types::*;
use std::collections::HashMap;

impl SimplexMethod {
    
    pub fn new(data: (ProblemKind, A, B, C, Operations)) -> Self {

        let n_vars = &data.1[0].len();

        let vars = vec!['a', 'e', 'h'];
        let mut var_positions: HashMap<char, Vec<usize>> = HashMap::new();

        for i in 0..vars.len() {
            var_positions.insert(vars[i], vec![]);
        }

        SimplexMethod {
            kind: data.0,
            should_terminate: false,
            a: data.1, // coeffs
            b: data.2, // results
            c: data.3, // z row
            operations: data.4,
            increased: Vec::new(),
            table: Vec::new(),
            pivot: (0, 0),
            two_fases: false,
            n_vars: *n_vars,
            var_positions,
            artificial_rows: Vec::new(),
            pivot_row_history: Vec::new(),
        }
    }

    // Obtener indices del pivot actual 
    // retorna una tupa con las coordenadas (i, j)

    fn get_pivot_indexes(&self) -> (usize, usize) {
        
        let mut div_vec = vec![];
        
        // Dependiendo del tipo de problema debemos seleccionar
        // la columna pivote de una forma u otra

        // Maximización => Menor negativo
        // Minimización => Mayor positivo

        let c_index = self.pivot_column();

        // Obtener vector auxiliar con los resultados
        // de la división entre col pivote y col de resultados

        // El menor valor de este vector seleccionará el indice de la fila pivote

        for i in 1..self.increased.len() {
            
            let col = self.increased[i][c_index];
            let res = self.increased[i][self.increased[0].len() - 1];

            div_vec.push(res / col);
        }

        let r_index = self.max_row_pivot(div_vec);

        (r_index, c_index)
    }

    fn pivot_column(&self) -> usize {
        
        let mut index = 0;
        let mut value = 0f64;

        let select = |x: f64, y: f64| x < y && x < 0f64;

        for i in 1..self.increased[0].len() - 1 {
            
            if select(self.increased[0][i], value) {
                value = self.increased[0][i];
                index = i;
            }
        }

        index
    }

    // Obtener indice de la fila pivote mediante el vector de division.
    // El valor seleccionado corresponde al menor >= 0 del vector.

    fn max_row_pivot(&self, div_vec: Vec<f64>) -> usize {
        
        let mut index = 0;
        let mut target = f64::INFINITY;

        let mut all_invalid = true;

        for (i, &value) in div_vec.iter().enumerate() {
            
            if value >= 0f64 && value < target && i + 1 != self.pivot.0 {
                all_invalid = false;
                index = i;
                target = value;
            }
        }

        // Si no se encuentra un valor positivo mayor a 0
        // o sea, son todo negativos o infinitos (x/0) el 
        // algoritmo debe terminar

        if all_invalid {
            self.get_shadow_prices();
            std::process::exit(0);
        }
        
        // Se retorna el indice + 1 ya que el conteo de filas se salta
        // la fila de la función objetivo (fila 0)

        index + 1
    }

    // Caso base de finalización
    // retorna false si encuentra un valor negativo, por lo tanto el algoritmo
    // continua, de lo contrario retorna true y el algoritmo finaliza

    fn should_finish(&mut self) -> bool {

        for i in 1..self.increased[0].len() - 1 {
            if self.increased[0][i] < 0f64 { return false }
        }

        true
    }

    // Pivoteo / eliminación gaussiana
    // se utilizan las funciones definidas anteriormente

    pub fn pivoting(&mut self) {

        while !self.should_finish() {

            let p_index = self.get_pivot_indexes();
            let mut increased = self.increased.clone();
            let pivot = increased[p_index.0][p_index.1];

            println!("Pivote: {} - Fila {} - Columna {}", &pivot, p_index.0, p_index.1);

            // Dividir fila pivote por pivote para hacer pivote = 1

            for value in increased[p_index.0].iter_mut() {
                *value /= pivot;
            }

            // Se itera sobre la matriz clonada y se aplica el pivoteo gaussiano

            for i in 0..increased.len() {
                
                if i == p_index.0 { continue }

                for j in 0..increased[i].len() {

                    increased[i][j] = self.increased[i][j] - 
                        (increased[p_index.0][j] * self.increased[i][p_index.1])
                    ;

                    if increased[i][j].abs() <= f64::EPSILON {
                        increased[i][j] = 0f64;
                    }
                }
            }

            // Actualizar parametros de la instacia y mostrar la tabla.
            
            self.pivot = p_index;
            self.increased = increased;

            self.update_table();
            self.print_table();
        }
    }

    pub fn check_valid_solution(&self) {

        if self.increased[0][self.increased[0].len()] != 0f64 {
            println!("El problema no es factible, ya que luego de la primera fase, el valor de la Z es distinto de 0\n");
            std::process::exit(1);
        }
    }

    pub fn get_shadow_prices(&self) {

        let h_indexes = self.var_positions.get(&'h').unwrap();

        println!("Precios sombra:\n");

        let mut h_count = 1;
        let mut h_values = vec![0f64, 0f64];

        for (i, num) in self.increased[0].iter().enumerate() {

            if h_indexes.contains(&i) {

                print!("h{} = {:.2}  ", h_count, num);

                if *num > h_values[0] {
                    h_values[0] = num.clone();
                    h_values[1] = h_count as f64;
                }
                
                h_count += 1;
            }
        }

        println!("\n\nDonde el precio sombra más rentable corresponde a la holgura h{}\n", h_values[1]);
    }

    pub fn normal_simplex(&mut self) {
        self.pivoting()
    }

    pub fn solve(&mut self) {
        
        self.to_increased_form();

        match self.two_fases {

            true => self.two_fases(),
            false => self.normal_simplex()
        }

        self.get_shadow_prices();

        std::process::exit(1);
    }
}
