
#![allow(dead_code)]

use std::thread;
use std::time::Duration;
use std::collections::HashMap;

use crate::types::*;

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
            a: data.1, // coeffs
            b: data.2, // results
            c: data.3, // z row
            operations: data.4, // Enum operation vector
            increased: Vec::new(), // increased form matrix
            table: Vec::new(), // String increased form
            pivot: (0, 0), // pivot coordinates (i, j)
            two_fases: false, // indicador si el problema es uno de 2 fases
            n_vars: *n_vars, // Cantidad de variables xn
            var_positions, // hashmap to allocate the variable positions
            artificial_rows: Vec::new(), // vector to allocate the artificial rows
            fase: 1,
        }
    }

    // Obtener indices del pivot actual 
    // retorna una tupa con las coordenadas (i, j)

    fn get_pivot_indexes(&self) -> (usize, usize) {
        
        let mut div_vec = vec![];
        
        // Dependiendo del tipo de problema debemos seleccionar
        // la columna pivote de una forma u otra
        //
        // Maximización => Menor negativo
        // Minimización => Mayor positivo

        let mut c_index = self.pivot_column_minimun();

        if self.fase == 2 && self.kind == ProblemKind::Minimize {
            c_index = self.pivot_column_largest();
        }

        // Obtener vector auxiliar con los resultados
        // de la división entre col pivote y col de resultados
        //
        // El menor valor de este vector seleccionará el indice de la fila pivote

        for i in 1..self.increased.len() {
            
            let col = self.increased[i][c_index];
            let res = self.increased[i][self.increased[0].len() - 1];

            div_vec.push(res / col);
        }

        println!("\nPivot vector aux: {:?}", div_vec);

        let r_index = self.max_row_pivot(div_vec);

        (r_index, c_index)
    }

    fn pivot_column_minimun(&self) -> usize {
        
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

    fn pivot_column_largest(&self) -> usize {

        let mut index = 0;
        let mut value = 0f64;

        let select = |x: f64, y: f64| x > y && x > 0f64;

        for i in 1..self.increased[0].len() - 1 {
            if select(self.increased[0][i], value) {
                value = self.increased[0][i];
                index = i;
            }
        }

        index
    }

    // Obtener indice de la fila pivote mediante el 
    // vector de division.
    //
    // El valor seleccionado corresponde al menor positivo del vector.

    fn max_row_pivot(&self, div_vec: Vec<f64>) -> usize {
        
        let mut index = 0;
        let mut target = f64::INFINITY;

        let mut all_invalid = true;

        for (i, &value) in div_vec.iter().enumerate() {
            
            if value > 0f64 && value < target {
                all_invalid = false;
                index = i;
                target = value;
            }
        }

        // Si no se encuentra un valor positivo mayor a 0
        // o sea, son todo negativos o infinitos (x/0) el 
        // algoritmo debe terminar

        if all_invalid {
            println!("No hay una fila pivote valida a seleccionar");
            std::process::exit(0);
        }

        // Se retorna el indice + 1 ya que el conteo de filas se salta
        // la fila de la función objetivo (fila 0)

        index + 1
    }

    // Caso base de finalización
    // retorna false si encuentra un valor negativo, por lo tanto el algoritmo
    // continua, de lo contrario retorna true y el algoritmo finaliza

    pub fn should_finish(&mut self) -> bool {

        for i in 1..self.increased[0].len() - 1 {
            if self.increased[0][i] < 0f64 { return false } else { continue }
        }

        true
    }

    // Pivoteo / eliminación gaussiana
    // se utilizan las funciones definidas anteriormente

    pub fn pivoting(&mut self) {

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
        
        thread::sleep(Duration::from_millis(1000));
    }

    fn one_fase(&mut self) {
        
        while !self.should_finish() {
            self.pivoting()
        }
    }

    pub fn solve(&mut self) {
        
        self.to_increased_form();

        match self.two_fases {
            true => self.two_fases(),
            false => self.one_fase(),
        }

        std::process::exit(1);
    }
}
