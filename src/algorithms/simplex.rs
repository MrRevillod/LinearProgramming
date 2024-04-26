
#![allow(dead_code)]

use crate::types::*;

impl SimplexMethod {

    pub fn new(data: (ProblemKind, A, B, C, Operations)) -> Self {

        SimplexMethod {
            n_vars: data.1[0].len(),
            kind: data.0,
            a: data.1, // coeffs
            b: data.2, // results
            c: data.3, // z row
            operations: data.4,
            increased: Vec::new(),
            pivot: (0, 0),
            two_fases: false,
            basic_vars: Vec::new(),
            basic_vars_history: Vec::new(),
            artificials_variables: Vec::new(),
            table: Vec::new(),
        }
    }

    // Obtener indices del pivot actual 
    // retorna una tupa con las coordenadas (i, j)

    fn get_pivot_indexes(&mut self) -> (usize, usize) {

        let pivot_column = self.get_pivot_column();
        let pivot_row = self.get_pivot_row(pivot_column.clone());

        self.basic_vars.push((pivot_row, pivot_column));
        self.basic_vars_history.push((pivot_row, pivot_column));

        (pivot_row, pivot_column)
    }

    fn get_pivot_column(&self) -> usize { // REVISADA

        let mut index = 1;
        let mut min = 0_f64;

        for i in 1..self.increased[0].len() - 1 {

            if self.increased[0][i] < min && self.increased[0][i] < 0_f64 {
                
                min = self.increased[0][i];
                index = i;
            }
        }

        index
    }

    // Obtener indice de la fila pivote mediante el vector de division.
    // El valor seleccionado corresponde al menor >= 0 del vector.

    fn get_min_div(&self, div_vec: Vec<f64>) -> usize {

        let mut index = 0;
        let mut target = f64::INFINITY;

        for (i, &value) in div_vec.iter().enumerate() {

            if value >= 0_f64 && value < target {
                index = i;
                target = value;
            }
        }

        // Se retorna el indice + 1 ya que el conteo de filas se salta
        // la fila de la función objetivo (fila 0)

        index + 1
    }

    fn get_pivot_row(&self, pivot_col: usize) -> usize {

        let mut div_vec = vec![];

        for i in 1..self.increased.len() {

            let col = self.increased[i][pivot_col];
            let res = self.increased[i][self.increased[0].len() - 1];

            div_vec.push(res / col);
        }

        let mut index = self.get_min_div(div_vec.clone());

        let mut is_basic_variable = false;

        for vb in self.basic_vars.iter() {

            if index == vb.0 {
                is_basic_variable = true;
                break;
            }
        }

        let mut max_iterations = 0;

        while is_basic_variable && max_iterations < self.a.len() {

            div_vec[index - 1] = -1.0;

            index = self.get_min_div(div_vec.clone());

            for vb in self.basic_vars.iter() {

                if index == vb.0 {
                    is_basic_variable = true;
                    break;
                }

                is_basic_variable = false;
            }

            max_iterations += 1;
        }

        if index == 0 { self.exit() }

        index
    }

    // Caso base de finalización
    // retorna false si encuentra un valor negativo, por lo tanto el algoritmo
    // continua, de lo contrario retorna true y el algoritmo finaliza

    fn should_finish(&mut self) -> bool {

        for i in 1..self.increased[0].len() - 1 {
            if self.increased[0][i] < 0_f64 { return false }
        }

        true
    }

    pub fn eliminacion_gauss(&self, increased: &mut Vec<Vec<f64>>, pivot: &(usize, usize)) {

        for i in 0..increased.len() {

            if i == pivot.0 { continue }

            for j in 0..increased[i].len() {

                increased[i][j] = self.increased[i][j] - 
                    (increased[pivot.0][j] * self.increased[i][pivot.1])
                ;

                if increased[i][j].abs() <= f64::EPSILON {
                    increased[i][j] = 0_f64;
                }
            }
        }
    }

    // Pivoteo / eliminación gaussiana
    // se utilizan las funciones definidas anteriormente

    pub fn pivoting(&mut self) {

        println!("Iniciando pivoteo ...");

        while !self.should_finish() {

            let p_index = self.get_pivot_indexes();
            let mut increased = self.increased.clone();
            let pivot = increased[p_index.0][p_index.1];

            println!("Pivote: {} - Fila {} - Columna {}", &pivot, p_index.0, p_index.1);

            // Dividir fila pivote por pivote para hacer pivote = 1

            for value in increased[p_index.0].iter_mut() {
                *value /= pivot;
            }

            self.eliminacion_gauss(&mut increased, &p_index);

            // Actualizar parametros de la instacia y mostrar la tabla.

            self.pivot = p_index;
            self.increased = increased;

            self.print_increased();

            // std::thread::sleep(std::time::Duration::from_millis(1000))
        }
    }

    pub fn check_valid_solution(&self) {

        if self.increased[0][self.increased[0].len()] != 0f64 {
            println!("El problema no es factible, ya que luego de la primera fase, el valor de la Z es distinto de 0\n");
            std::process::exit(1);
        }
    }

    pub fn normal_simplex(&mut self) {
        self.print_increased();
        self.pivoting();
        self.print_increased();
    }

    pub fn solve(&mut self) {
        
        self.make_initial_table();

        self.to_increased_form();

        match self.two_fases {

            true => self.two_fases(),
            false => self.normal_simplex()
        }

        println!("{:?}", self.table);

        std::process::exit(1);
    }

    pub fn exit(&self) {
        println!("El programa ha finalizado");
        std::process::exit(1);
    }
}
