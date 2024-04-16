
// let mut add_table_vars = |c: char, count: &mut usize| {
//     self.table[0].push(format!("{}{}", c, count));
//     *count += 1;
// };
//

// Add aditional vars to A Matrix (coeffs)

let mut add_a_vars = |value: f64| {
    
    for j in 0..temp_a.len() {
        temp_a[j].push(conditional!(i == j ? value: 0f64));
    }
};

// Add aditional vars to Z row

// let mut add_z_vars = |value: f64| self.c.push(value);


match value {

    Operation::Gt => {

        // add_a_vars(-1f64); // - e_n
        // add_a_vars(1f64);  // + a_n
        //
        // add_z_vars(1f64);  // z row => e_n
        // add_z_vars(0f64);  // z row => a_n
        // 
        // add_table_vars('e', &mut e_count);
        // add_table_vars('a', &mut a_count);

        self.add_variable('e', &mut e_count, &mut temp_a, &i);
        self.add_variable('a', &mut a_count, &mut temp_a, &i);

        self.two_fases = true;
    },

    Operation::Lt | Operation::Eq => {

        // add_a_vars(1f64); // + h_n
        // add_z_vars(0f64); // z row => h_n
        // add_table_vars('h', &mut h_count);

        self.add_variable('h', &mut h_count, &mut temp_a, &i);
    }
}

