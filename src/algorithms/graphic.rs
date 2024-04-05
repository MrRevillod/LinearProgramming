
use crate::types::*;
use crate::linear::*;

impl GraphicMethod {

    pub fn new(data: (ProblemKind, A, B, Z, Operations)) -> Self {

        GraphicMethod {
            kind: data.0,
            a: data.1,
            b: data.2,
            z: data.3,
            operations: data.4,
            intersections: Intersections::new(),
            optimal_point: Point { x: 0.0, y: 0.0 },
            utility: 0.0
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
                self.optimal_point = Point::new(x.clone(), y.clone());
            }
        }
    }

    pub fn solve(&mut self) {

        self.get_all_intersections();
        self.get_feasible_intersections();
        self.optimize();

        dbg!(&self);
    }
}
