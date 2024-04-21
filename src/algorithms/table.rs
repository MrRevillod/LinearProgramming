
#![allow(dead_code)]

use conditional::conditional;

use crate::types::*;

impl SimplexMethod {

    pub fn init_increased_table(&mut self) {

        let mut header = vec![String::from("VB"), String::from("Z")];

        for i in 0..self.n_vars {
            header.push(format!("x{}", i + 1));
        }

        self.table.push(header);

        for _i in 0..self.a.len() + 1 {
            self.table.push(vec![])
        }

        self.table[0].push(String::from("LD"));
        self.table[1].push(String::from("Z"));
    }

    pub fn init_sec_fase_table(&mut self) {

        let mut table = vec![
            vec![String::from("VB"), String::from("Z")],
            vec![String::from("Z")],
        ];

        for i in 0..self.n_vars {
            table[0].push(format!("x{}", i + 1));
        }

        for i in 1..self.var_positions.get(&'e').unwrap().len() + 1 {
            table[0].push(format!("e{}", i));
        }
        
        for i in 1..self.var_positions.get(&'h').unwrap().len() + 1 {
            table[0].push(format!("h{}", i));
        }

        for i in 2..self.table.len() {
            table.push(vec![self.table[i][0].clone()])
        }

        table[0].push(String::from("LD"));

        self.table = table;
    }

    pub fn add_variable(&mut self, c: char, count: &mut usize, a: &mut A, iter: &usize) {

        // Update table variables
        // values = ( a var value, z var value )
        
        let values: (f64, f64) = match &c { 
            'h' => (1.0, 0.0),
            'a' => (1.0, 1.0),
            'e' => (-1.0, 0.0),
            _   => panic!("Invalid var type")
        };

        let var_name = format!("{}{}", &c, &count);

        self.table[0].pop();
        self.table[0].push(var_name.clone());

        self.var_positions.get_mut(&c).unwrap().push(self.table[0].len() - 2);


        self.table[0].push(String::from("LD"));

        // add the basic variable to the first column

        if c == 'a' || c == 'h' {
            
            for (i, row) in self.table.iter_mut().enumerate() {

                if row.is_empty() {

                    if c == 'a' {
                        self.artificial_rows.push(i - 1);
                    }

                    row.push(var_name.clone());
                    break
                }
            }
        }

        *count += 1;

        // Update matrix variables

        for j in 0..a.len() {
            a[j].push(conditional!(*iter == j ? values.0: 0f64));
        }

        self.c.push(values.1);
    }
    
    pub fn to_increased_form(&mut self) {

        self.init_increased_table();

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

        self.increased.insert(0, self.c.clone()); // push the z row (c) into increased

        if self.two_fases {
            
            for i in 1..self.n_vars + 1 {
                self.increased[0][i] = 0f64
            }

        } else {
            
            for i in 1..self.n_vars + 1 {
                self.increased[0][i] *= -1f64
            }
        }

        for i in 0..self.increased.len() { // add the result (b) column
            self.increased[i].push(self.b[i].clone()) // to the increased form matrix
        }

        println!("\nForma aumentada inicial...");
        
        self.update_table();
        self.print_table();

        println!("{:?}", self.artificial_rows);
    }

    pub fn update_table(&mut self) {

        if self.pivot != (0, 0) {
            self.table[self.pivot.0 + 1][0] = self.table[0][self.pivot.1 + 1].clone();
        }

        for i in 1..self.increased.len() + 1 {

            for j in 1..self.increased[0].len() + 1{

                if self.table[i].len() < self.increased[0].len() {
                    self.table[i].resize(self.increased[0].len() + 1, String::new());
                }

                self.table[i][j] = self.increased[i - 1][j - 1].to_string();
                // self.table[i][j] = parse_to_frac(&self.table[i][j]);
            }
        }
    }

    pub fn print_table(&self) {
        
        println!();

        for row in self.table.iter() {

            for item in row {
                print!("{:<8}", format!("{:.5}", item));
            }

            println!();
        }

        println!();
    }

    pub fn print_increased(&self) {
        
        println!();

        for row in self.increased.iter() {

            for item in row {
                print!("{:<8}", format!("{:.2}", item));
            }

            println!();
        }

        println!();
    }

}

