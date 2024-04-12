
use conditional::conditional;

use crate::linear::{gen_fases_identity, gen_identity};
use crate::types::*;

impl SimplexMethod {
    
    pub fn new(data: (ProblemKind, A, B, C, Operations)) -> Self {
        SimplexMethod {
            kind: data.0,
            a: data.1,
            b: data.2,
            c: data.3,
            operations: data.4,
            increased: Vec::new(),
            table: Vec::new(),
            pivot: (0, 0),
        }
    }

    fn to_increased_form(&mut self) {

        // Complete the matrix with h ; a ; e

        // 2 fases => Preparar ecuaciones:
        //
        // <= : + holgura_n
        // >= : - exceso_n + artificial_n
        // =  : + artificial_n

        for (i, value) in self.operations.iter().enumerate() {

            let mut add_variables = |value: f64| {
                
                for j in 0..self.a.len() {
                    self.a[j].push(conditional!(i == j ? value: 0f64));
                }
            };
            
            match value {
                Operation::Gt => {
                    add_variables(-1f64); add_variables(1f64);
                },
                Operation::Lt | Operation::Eq => {
                    add_variables(1f64)
                }
            }
        }

        // End of completion
        
        // Begin populate Z Column and row
        
        self.a.iter_mut().for_each(|x| x.insert(0, 0f64));

        match self.kind {
            
            ProblemKind::Minimize => {
                self.c.insert(0, -1f64);
            },

            ProblemKind::Maximize => {
                self.c.iter_mut().for_each(|x| *x *= -1.0);
                self.c.insert(0, 1f64);
            }
        }

        for row in self.a.iter() {
            
            for item in row {
                print!("{:<8}", item);
            }

            println!();
        }

        std::process::exit(1);
    }

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

        self.init_table();
        self.print_table();

        while !self.should_finish() {
            
            let p_index = self.get_pivot_indexes();
            let mut increased = self.increased.clone();

            let pivot = increased[p_index.0][p_index.1];

            for value in increased[p_index.0].iter_mut() {
                *value /= pivot;
            }

            for i in 0..increased.len() {
                
                if i == p_index.0 { continue }

                for j in 0..increased[i].len() {
increased[i][j] = self.increased[i][j] - increased[p_index.0][j] * self.increased[i][p_index.1];
                }
            }

            self.pivot = p_index;
            self.increased = increased;

            self.update_table();
            self.print_table();
        }
    }

    fn init_table(&mut self) {
        
        let mut table: Vec<Vec<String>> = vec![];
        let mut side: Vec<String> = vec!["Z".to_string()];
        let mut head: Vec<String> = vec!["VB".to_string(), "Z".to_string()];

        for i in 1..self.a[0].len() + 1 {
            head.push(format!("x{}", i));
        }

        for i in 1..self.a.len() + 1 {
            head.push(format!("h{}", i));
            side.push(format!("h{}", i));
        }

        head.push("LD".to_string());
        table.push(head);

        for (i, row) in self.increased.iter().enumerate() {
            let mut new_row = Vec::new();
            new_row.push(side[i].clone());

            for &item in row.iter() {
                new_row.push(format!("{:.1}", item));
            }

            table.push(new_row);
        }

        self.table = table;
    }

    fn update_table(&mut self) {
        
        self.table[self.pivot.0 + 1][0] = self.table[0][self.pivot.1 + 1].clone();

        for (i, row) in self.table.iter_mut().enumerate() {
            
            if i == 0 { continue }

            for (j, item) in row.iter_mut().enumerate() {
                
                if j == 0 { continue }

                *item = format!("{:.1}", self.increased[i - 1][j - 1]);
            }
        }
    }

    fn print_table(&self) {
        
        println!();

        for row in self.table.iter() {
            for item in row {
                print!("{:<8}", item);
            }

            println!();
        }

        println!();
    }
}
