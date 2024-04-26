
#![allow(dead_code)]

use conditional::conditional;

use crate::types::*;

impl SimplexMethod {

    pub fn make_initial_table(&mut self) {

        let mut header = vec![String::from("VB"), String::from("Z")];

        for i in 1..self.n_vars + 1 {
            header.push(format!("x{}", i))
        }

        header.push(String::from("LD"));

        let sidebar = vec![String::from("Z")];

        self.table.push(header);
        self.table.push(sidebar);

        println!("{:?}", self.table);
    }

    pub fn add_variable(&mut self, c: char, count: &mut usize, a: &mut A, iter: &usize) {

        // Update increased form variables
        // values = ( a var value, z var value )

        let values: (f64, f64) = match &c { 
            'h' => (1.0, 0.0),
            'a' => (1.0, 1.0),
            'e' => (-1.0, 0.0),
            _   => panic!("Invalid var type")
        };

        for j in 0..a.len() {
            a[j].push(conditional!(*iter == j ? values.0: 0f64));
        }
        
        if c == 'a' {
            self.artificials_variables.push((*iter + 1, a[0].len())) 
        }

        self.c.push(values.1);

        // Add variable to the table (header && sidebar)

        self.table[0].pop();
        self.table[0].push(format!("{}{}", c, count));
        self.table[0].push(String::from("LD"));

        *count += 1;
    }
    
    pub fn to_increased_form(&mut self) {

        let mut temp_a = self.a.clone();
        let (mut a_count, mut e_count, mut h_count) = (1, 1, 1);

        for (i, operation) in self.operations.clone().iter().enumerate() {

            match operation {

                Operation::Gt => {
                    self.two_fases = true;
                    self.add_variable('e', &mut e_count, &mut temp_a, &i);
                    self.add_variable('a', &mut a_count, &mut temp_a, &i);
                },

                Operation::Lt | Operation::Eq => {
                    self.add_variable('h', &mut h_count, &mut temp_a, &i);
                }
            }
        }

        self.increased = temp_a; // Create the increased form matrix

        for row in self.increased.iter_mut() {
            row.insert(0, 0f64); // add the initial 0 column
        }

        match self.kind {
            ProblemKind::Maximize => self.c.insert(0, 1f64),
            ProblemKind::Minimize => self.c.insert(0, -1f64),
        }

        self.increased.insert(0, self.c.clone()); // push the z row (c) into increased

        match self.kind {

            ProblemKind::Maximize => {

                for i in 1..self.n_vars + 1 {
                    self.increased[0][i] *= -1f64
                }
            },

            ProblemKind::Minimize => {}
        }

        self.b.insert(0, 0f64);

        for i in 0..self.increased.len() { // add the result (b) column
            self.increased[i].push(self.b[i].clone()) // to the increased form matrix
        }

        println!("\nForma aumentada inicial ...");

        self.print_increased();
    }

    pub fn print_increased(&self) {
        
        println!();

        for row in self.increased.iter() {

            for item in row {
                print!("{:<10}", format!("{:.4}", item));
            }

            println!();
        }
        
        println!();
    }

}

