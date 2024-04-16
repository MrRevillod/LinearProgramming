
use crate::types::*;

impl SimplexMethod {
    
    pub fn new(data: (ProblemKind, A, B, C, Operations)) -> Self {

        let n_vars = &data.1[0].len();

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
            n_vars: *n_vars
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

    fn should_finish(&self) -> bool {
        
        for &value in self.increased[0].iter() {
            if value < 0f64 { return false } else { continue }
        }

        true
    }

    pub fn solve(&mut self) {
        
        self.to_increased_form();

//         self.init_table();
//         self.print_table();
//
//         while !self.should_finish() {
//             
//             let p_index = self.get_pivot_indexes();
//             let mut increased = self.increased.clone();
//
//             let pivot = increased[p_index.0][p_index.1];
//
//             for value in increased[p_index.0].iter_mut() {
//                 *value /= pivot;
//             }
//
//             for i in 0..increased.len() {
//                 
//                 if i == p_index.0 { continue }
//
//                 for j in 0..increased[i].len() {
// increased[i][j] = self.increased[i][j] - increased[p_index.0][j] * self.increased[i][p_index.1];
//                 }
//             }
//
//             self.pivot = p_index;
//             self.increased = increased;
//
//             self.update_table();
//             self.print_table();
//         }
    }


}
