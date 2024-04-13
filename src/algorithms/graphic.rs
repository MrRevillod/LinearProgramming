
use crate::types::*;
use crate::linear::*;

use inline_python::python;

impl GraphicMethod {

    pub fn new(data: (ProblemKind, A, B, Z, Operations, Vec<Vec<f64>>)) -> Self {

        GraphicMethod {
            kind: data.0,
            a: data.1,
            b: data.2,
            z: data.3,
            operations: data.4,
            intersections: Intersections::new(),
            python_intersections: Vec::new(),
            optimal_point: [0f64, 0f64],
            utility: 0.0,
            inequalities: data.5,
        }
    }

    pub fn get_all_intersections(&mut self) {

        let mut intersections = Intersections::new();

        for i in 0..self.a.len() {

            for j in i+1..self.a.len() {

                let pair_of_eq = vec![
                    self.a[i].clone(),
                    self.a[j].clone()
                ];

                let pair_of_res = vec![self.b[i], self.b[j]];
                let intersection = cramer(&pair_of_eq, &pair_of_res);

                match intersection {
                    Ok(point) => intersections.push(point),
                    Err(_) => continue
                }
            }
        }

        self.intersections = intersections;
    }

    pub fn get_feasible_intersections(&mut self) {

        let mut intesections = Intersections::new();

        'outer: for Point { x, y } in self.intersections.iter() {

            for i in 0..self.a.len() {

                let is_valid = match self.operations[i] {
                    Operation::Gt => self.a[i][0] * x + self.a[i][1] * y >= self.b[i],
                    Operation::Lt => self.a[i][0] * x + self.a[i][1] * y <= self.b[i],
                    Operation::Eq => self.a[i][0] * x + self.a[i][1] * y == self.b[i],
                };

                if !is_valid {
                    continue 'outer
                }
            }

            intesections.push(
                Point::new(x.clone(), y.clone())
            )
        }

        self.intersections = intesections;
    }

    pub fn optimize(&mut self) {

        let optimize = match self.kind {
            ProblemKind::Minimize => |a: f64, b: f64| a < b,
            ProblemKind::Maximize => |a: f64, b: f64| a > b,
        };

        for Point { x, y } in self.intersections.iter() {

            let utility = self.z[0] * x + self.z[1] * y;

            if optimize(utility, self.utility) {
                self.utility = utility;
                self.optimal_point = [x.clone(), y.clone()];
            }
        }
    }

    pub fn prepare_for_graphic(&mut self) {
        
        let mut points = Vec::new();
        let mut intersections = self.intersections.clone();

        let dist = |uno: &Point, dos: &Point| {
            ((dos.x - uno.x).powf(2f64) + (dos.y - uno.y).powf(2f64)).sqrt()
        };

        let mut current_point = intersections.remove(0);
        points.push(vec![current_point.x.clone(), current_point.y.clone()]);

        while !intersections.is_empty() {

            intersections.sort_by(
                |a, b| dist(&current_point, a).partial_cmp(&dist(&current_point, b)).unwrap()
            );

            current_point = intersections.remove(0);
            points.push(vec![current_point.x.clone(), current_point.y.clone()]);
        }

        self.python_intersections = points;

        self.inequalities.pop();
        self.inequalities.pop();
    }

    pub fn graphic(&self) {

        let intersections = self.python_intersections.clone();
        let inequalities = self.inequalities.clone();
        let optimal_point = self.optimal_point.clone();

        // aquí llamar a matplotlib y graficar 

        python! {

            intersections = 'intersections
            inequalities = 'inequalities
            optimal_point = 'optimal_point

            print(intersections)
            print(inequalities)
            print(optimal_point)
        }
    }

    pub fn solve(&mut self) {
        
        self.get_all_intersections();
        self.get_feasible_intersections();
        self.optimize();
        self.prepare_for_graphic();
        self.graphic();
    }
}
