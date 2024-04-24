
use crate::parser::Problem;
use crate::lexer::TokenType;

fn should_finish(z_array: Vec<f64>) -> bool {
    let mut i = 1;
    while i < (z_array.len() - 1) {
        // if z_array[i].powf(11.0).trunc() < 0.0 { // descomentar para numeros de coma flotante
        if z_array[i] < 0.0 {
            return false;
        }
        i += 1;
    }

    true
}


fn get_x_pivot(vector: Vec<Vec<f64>>) -> usize {

    let mut index = 1;
    let mut min = vector[0][index];

    let mut all_equal = true;


    for i in 2..(vector[0].len() - 1) {
        if vector[0][i] < min && vector[0][i] < 0.0 {
            min = vector[0][i];
            index = i;
        }
        if min != vector[0][i] && vector[0][i] < 0.0 {
            all_equal = false;
        }
    }
    if all_equal {
        let mut indexes = Vec::new();
        for i in 1..(vector[0].len() - 1) {
            if vector[0][i] >= 0.0 { continue; }
            indexes.push(i);
        }

        let mut divs_vector = Vec::new();

        for i in indexes.iter() {
            let mut div = Vec::new();
            for j in 1..vector.len() {
                div.push(vector[j][vector[0].len() - 1] / vector[j][*i]);
            }
            divs_vector.push((i, div));
        }


        let mut minor_vector = Vec::new();
        for i in divs_vector {

            let mut min = i.1[0];
            for j in 1..i.1.len() {
                if min > i.1[j] && i.1[j] > 0.0 {
                    min = i.1[j];
                }
            }

            minor_vector.push((i.0, min));
        }
        

        let mut max_index = 0;

        for i in 0..minor_vector.len() {
            if minor_vector[i].1 > minor_vector[max_index].1 {
                max_index = i;
            }
        }
        index = *minor_vector[max_index].0;
    }

    return index;
}

fn get_min_div(div_vector: Vec<f64>) -> usize {
    let mut min = ((1 as usize) << 63) as f64;
    let mut index = 0;

    for i in 0..div_vector.len() {
        if div_vector[i] < min && div_vector[i] > 0.0 {
            min = div_vector[i];
            index = i;
        }
    }

    index + 1
}

fn get_y_pivot(matrix: Vec<Vec<f64>>, x_pivot: usize) -> usize {
    
    let mut div_vector = Vec::new();

    let n_size = matrix[0].len();

    for i in 1..matrix.len() {
        div_vector.push(matrix[i][n_size - 1] / matrix[i][x_pivot]);
    }

    let index = get_min_div(div_vector.clone());

    if index == 0 {
        std::process::exit(1);
    }

    index
}

fn get_pivot_indexes(matrix: &Vec<Vec<f64>>) -> (usize, usize) {
    let x_pivot = get_x_pivot(matrix.clone());
    let y_pivot = get_y_pivot(matrix.clone(), x_pivot);

    (y_pivot, x_pivot)
}


fn pivoting(matrix1: Vec<Vec<f64>>, matrix2: &mut Vec<Vec<f64>>, x_pivot: usize, y_pivot: usize) {

    let n_size = matrix1[y_pivot].len();
    let m_size = matrix1.len();

    for j in 0..n_size {
        matrix2[y_pivot][j] = matrix1[y_pivot][j] / matrix1[y_pivot][x_pivot];
    }

    for i in 0..m_size {
        if i == y_pivot { continue; }
        if matrix1[i][x_pivot] == 0.0 {
            for j in 0..n_size {
                matrix2[i][j] = matrix1[i][j];
            }
            continue;
        }
        for j in 0..n_size {
            matrix2[i][j] = matrix1[i][j] - (matrix2[y_pivot][j] * matrix1[i][x_pivot]);
        }
    }
}


fn initial_matrix(problem: Problem) -> Vec<Vec<f64>> {
    let n_vars = problem.func.vars.len();
    let m_size = problem.constrains.len() - 1;
    let mut n_size = 0;

    for constrain in &problem.constrains {
        if constrain.y == 0.0 { continue; }
        n_size += 1;
    }

    n_size += n_vars + 2;
    let n_size = n_size;
    let mut matrix = Vec::<Vec<f64>>::with_capacity(m_size);

    for i in 0..m_size {
        matrix.push(Vec::<f64>::with_capacity(n_size));
        for _j in 0..n_size {
            matrix[i].push(0.0);
        }
    }

    for i in 0..n_vars {
        (matrix[0][i + 1], matrix[0][0]) = match problem.obj {
            TokenType::Max =>  (-problem.func.vars[
                format!("x{}", (i + 1)).as_str()
            ], 1.0),

            _ =>  (problem.func.vars[
                format!("x{}", (i + 1)).as_str()
            ], -1.0),
        }
    }

    for i in (n_vars + 1)..(n_size - 1) {
        for j in 1..m_size {
            if j + n_vars == i {
                matrix[j][i] = 1.0;
            }
        }
    }

    let mut i = 1;
    let mut index_constrain = 0;
    while i < m_size {
        if problem.constrains[index_constrain].y == 0.0 {
            index_constrain += 1;
            continue;
        }

        for j in 0..n_vars {
            matrix[i][j + 1] = problem.constrains[index_constrain].coefs[
                format!("x{}", (j + 1)).as_str()
            ];
        }

        matrix[i][n_size - 1] = problem.constrains[index_constrain].y;
        index_constrain += 1;
        i += 1;
    }

    matrix
}

fn initial_matrix_first_fase(problem: &mut Problem) -> Vec<Vec<f64>> {
    let n_vars = problem.func.vars.len();
    let n_constrains = problem.constrains.len();
    let m_size = n_constrains + 1;

    let mut n_size = 2 + n_vars;

    let mut n_artificial = 0;
    let mut n_holgura = 0;
    let mut n_exceso = 0;

    for constrain in &problem.constrains {
        if constrain.sig == TokenType::Gequal {
            n_artificial += 1;
            n_exceso += 1;
        }
        if constrain.sig == TokenType::Lequal {
            n_holgura += 1;
        }
    }


    for _i in 0..m_size {
        problem.coefs_indexes.push(0);
    }

    for _i in 0..n_vars {
        problem.coefs.push(0.0);
    }

    for i in 0..n_vars {
        problem.coefs[i] = problem.func.vars[
                format!("x{}", (i + 1)).as_str()
            ];
    }

    let n_new_vars = n_holgura + n_artificial + n_exceso;
    n_size += n_new_vars;
    let n_size = n_size;

    let mut matrix = Vec::<Vec<f64>>::with_capacity(m_size);

    for i in 0..m_size {
        matrix.push(Vec::<f64>::with_capacity(n_size));
        for _j in 0..n_size {
            matrix[i].push(0.0);
        }
    }

    matrix[0][0] = -1.0;

    let mut i = 1;
    let mut index_constrain = 0;
    let mut index_new_vars = 0;
    while i < m_size {
        let mut j = 0;

        while j < n_vars {
            matrix[i][j + 1] = problem.constrains[index_constrain].coefs[
                format!("x{}", (j + 1)).as_str()
            ];
            j += 1;
        }

        if problem.constrains[index_constrain].sig == TokenType::Lequal {
            matrix[i][j + 1 + index_new_vars] = 1.0;
            index_new_vars += 1;
        }
        if problem.constrains[index_constrain].sig == TokenType::Gequal {
            matrix[i][j + 1 + index_new_vars] = -1.0;
            index_new_vars += 1;
            matrix[i][j + 1 + index_new_vars] = 1.0;
            problem.indexes_a.push(vec![i, j + 1 + index_new_vars]);
            matrix[0][j + 1 + index_new_vars] = 1.0;
            index_new_vars += 1;
        }

        matrix[i][n_size - 1] = problem.constrains[index_constrain].y;
        index_constrain += 1;
        i += 1;
    }

    for i in problem.indexes_a.iter() {
        for j in 0..matrix[0].len() {
            matrix[0][j] += matrix[i[0]][j] * -1.0;
        }
    }

    matrix
}

fn first_fase(problem: &mut Problem) -> Vec<Vec<f64>> {
    println!("Primera fase:");
    let mut matrix = initial_matrix_first_fase(problem);

    let mut c_matrix = 0;
    let mut l_matrix = 1;
    let mut matrix1 = matrix.clone();

    let mut s_matrix = vec![&mut matrix, &mut matrix1];

    for x in s_matrix[c_matrix].clone() {
        println!("{:?}", x);
    }

    while !should_finish(s_matrix[c_matrix][0].clone()) {

        let (y_pivot, x_pivot) = get_pivot_indexes(s_matrix[c_matrix]);

        problem.coefs_indexes[y_pivot] = x_pivot;

        pivoting(s_matrix[c_matrix].to_vec(), &mut s_matrix[l_matrix], x_pivot, y_pivot);

        let swap = c_matrix;
        c_matrix = l_matrix;
        l_matrix = swap;

        println!("");
        for x in s_matrix[c_matrix].clone() {
            println!("{:?}", x);
        }
    }

    let n_size = s_matrix[c_matrix][0].len();

    if s_matrix[c_matrix][0][n_size -1] != 0.0 {
        println!("{}: El problema no tiene solución", s_matrix[c_matrix][0][n_size -1]);
        std::process::exit(1);
    }

    s_matrix[c_matrix].to_vec()
}

fn initial_matrix_second_fase(problem: Problem, matrix: &mut Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n_vars = problem.func.vars.len();

    for i in 0..matrix.len() {
        for j in 0..problem.indexes_a.len() {
            matrix[i][problem.indexes_a[j][1]] = 0.0;
        }
    }

    matrix[0][0] = 1.0;
    for i in 0..n_vars {
        matrix[0][i + 1] = match problem.obj {
            TokenType::Max =>  -problem.func.vars[
                format!("x{}", (i + 1)).as_str()
            ],

            _ =>  problem.func.vars[
                format!("x{}", (i + 1)).as_str()
            ],
        }
    }

    let mut index = 0;
    for i in 0..problem.coefs_indexes.len() {
        if problem.coefs_indexes[i] != 0 {
            let pivot = matrix[0][problem.coefs_indexes[i]];
            for j in 1..matrix[0].len() {
                matrix[0][j] -= pivot * matrix[i][j];
            }
            println!("");
        }
    }

    for x in matrix.clone() {
        println!("{:?}", x);
    }
    matrix.to_vec()
}

fn second_fase(problem: Problem, matrix: &mut Vec<Vec<f64>>) {
    println!("Segunda fase:");

    let mut matrix = initial_matrix_second_fase(problem.clone(), matrix);
    // std::process::exit(1);
    let mut c_matrix = 0;
    let mut l_matrix = 1;
    let mut matrix1 = matrix.clone();

    let mut s_matrix = vec![&mut matrix, &mut matrix1];

    for x in s_matrix[c_matrix].clone() {
        println!("{:?}", x);
    }

    while !should_finish(s_matrix[c_matrix][0].clone()) {

        let (y_pivot, x_pivot) = get_pivot_indexes(s_matrix[c_matrix]);


        pivoting(s_matrix[c_matrix].to_vec(), &mut s_matrix[l_matrix], x_pivot, y_pivot);

        let swap = c_matrix;
        c_matrix = l_matrix;
        l_matrix = swap;

        println!("");
        for x in s_matrix[c_matrix].clone() {
            println!("{:?}", x);
        }
    }

}

pub fn simplex(problem: &mut Problem) {
    let mut are_a_vars = false;

    for constrain in problem.constrains.iter() {
        if (constrain.sig == TokenType::Gequal ||
            constrain.sig == TokenType::Equal  ||
            constrain.sig == TokenType::Gthan) && constrain.y != 0.0 {
            are_a_vars = true;
        }
    }

    if are_a_vars {
        println!("Método simplex en 2 fases:");

        let mut matrix_first_fase = first_fase(problem);

        second_fase(problem.clone(), &mut matrix_first_fase);


    } else {
        println!("Método simplex:");
        let mut matrix = initial_matrix(problem.clone());

        let mut c_matrix = 0;
        let mut l_matrix = 1;
        let mut matrix1 = matrix.clone();

        let mut s_matrix = vec![&mut matrix, &mut matrix1];

        while !should_finish(s_matrix[c_matrix][0].clone()) {

            let (y_pivot, x_pivot) = get_pivot_indexes(s_matrix[c_matrix]);

            pivoting(s_matrix[c_matrix].to_vec(), &mut s_matrix[l_matrix], x_pivot, y_pivot);

            let swap = c_matrix;
            c_matrix = l_matrix;
            l_matrix = swap;

        }
        for x in s_matrix[c_matrix].clone() {
            println!("{:?}", x);
        }
    }
}
