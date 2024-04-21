
#![allow(dead_code)]

use std::thread;
use std::time::Duration;
use std::collections::HashMap;

use crate::linear::casi_cero;
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
            operations: data.4,
            increased: Vec::new(),
            table: Vec::new(),
            pivot: (0, 0),
            two_fases: false,
            n_vars: *n_vars,
            var_positions,
        }
    }

    // Complete the matrix with h ; a ; e

    // 2 fases => Preparar ecuaciones:
    //
    // <= : + holgura_n
    // >= : - exceso_n + artificial_n

    fn get_pivot_indexes(&self) -> (usize, usize) {
        
        let mut div_vec = vec![];
        let c_index = self.pivot_column();

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

    fn max_row_pivot(&self, div_vec: Vec<f64>) -> usize {
        
        let mut index = 0;
        let mut target = f64::INFINITY;

        for (i, &value) in div_vec.iter().enumerate() {
            
            if value > 0f64 && value < target {
                index = i;
                target = value;
            }
        }

        index + 1
    }

    fn should_finish(&mut self) -> bool {

        println!("{:?}", self.increased[0]);

        for i in 1..self.increased[0].len() - 1 {

            // self.increased[0][i] = self.truncar_a_decimales(self.increased[0][i], 3);
            // self.increased[0][i] = self.redondear_a_decimales(self.increased[0][i], 2);

            // if self.increased[0][i] < 0f64 { return false } else { continue }
            if casi_cero(&mut self.increased[0][i].clone()) { return false } else { continue }
        }

        true
    }

    fn pivoting(&mut self) {

        while !self.should_finish() {

            let p_index = self.get_pivot_indexes();
            let mut increased = self.increased.clone();
            let pivot = increased[p_index.0][p_index.1];

            println!("Pivote: {} - Fila {} - Columna {}", &pivot, p_index.0, p_index.1);

            // Dividir fila pivote por pivote para hacer pivote = 1
            for value in increased[p_index.0].iter_mut() {
                *value /= pivot;
            }

            for i in 0..increased.len() {
                
                if i == p_index.0 { continue }

                for j in 0..increased[i].len() {

                    increased[i][j] = self.increased[i][j] - 
                        increased[p_index.0][j] * self.increased[i][p_index.1]
                    ;

                    if increased[i][j].abs() <= f64::EPSILON {
                        increased[i][j] = 0f64;
                    }
                }
            }
            
            self.pivot = p_index;
            self.increased = increased;

            self.update_table();
            self.print_table();
            
            thread::sleep(Duration::from_millis(1000));
        }
    }


    pub fn two_fases(&mut self) {

        println!("Iniciando primera fase ...");

        for i in 1..self.increased.len() {

            for j in 0..self.increased[0].len() {
                self.increased[0][j] += self.increased[i][j] * -1f64
            }
        }

        println!("\nFilas restadas en fila z...");

        self.update_table();
        self.print_table();

        println!("\nIniciando pivoteo...");

        self.pivoting();
        
        println!("Iniciando segunda fase ...");

        // Crear matríz sin variables artificiales
        // self.var_positions => diccionario con las posiciones de las variables

        let mut new_increased = Vec::new();

        for i in 0..self.increased.len() {

            let mut row = Vec::new();

            for j in 0..self.increased[0].len() {

                if !self.var_positions.get(&'a').unwrap().contains(&j) {

                    row.push(self.increased[i][j])
                }
            }

            new_increased.push(row);
        }

        std::process::exit(1);

        for i in 1..self.n_vars + 1 {
            new_increased[0][i] = self.c[i].clone() * -1_f64;
        }

        self.pivot = (0, 0);
        self.increased = new_increased;

        self.init_sec_fase_table();
        self.update_table();
        self.print_table();

        self.pivoting();
    }

    pub fn solve(&mut self) {
        
        self.to_increased_form();

        match self.two_fases {
            true => self.two_fases(),
            false => self.pivoting()
        }

        std::process::exit(1);
    }
}
