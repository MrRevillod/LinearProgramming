
#![allow(dead_code)]

use crate::types::*;
use crate::linear::*;
// use inline_python::python;

impl GraphicMethod {

    pub fn new(data: (ProblemKind, A, B, Z, Operations)) -> Self {

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
            inequalities: Vec::new(),
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

            intesections.push(Point::new(x.clone(), y.clone()))
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

        for i in 0..self.a.len() {
            self.inequalities.push(
                vec![self.b[i].clone(), self.a[i][0].clone(), self.a[i][1].clone()]
            )
        }
        
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

    #[allow(warnings)]
    pub fn graphic(&self) {

        let intersections = self.python_intersections.clone();
        let inequalities = self.inequalities.clone();
        let optimal_point = self.optimal_point.clone();
        let z = self.z.clone();

        // python! {
        //
        //     import matplotlib.pyplot as plt
        //     from matplotlib.patches import Polygon
        //     import numpy as np
        //     import matplotlib.animation as animation
        //
        //     def pyformat(x, i):
        //         return x+str(i)
        //
        //     intersections = 'intersections
        //     inequalities = 'inequalities
        //     optimal_point = 'optimal_point
        //     z = 'z
        //
        //     polygon = Polygon(intersections, closed=True, fill=True, color="red", alpha=0.3)
        //
        //     domain_animation = np.arange(round(-optimal_point[0]*30), round(optimal_point[1]*30), 0.1)
        //
        //     operations = {}
        //
        //     for i, p in enumerate(inequalities):
        //         if p[2] == 0:
        //             operations[pyformat("x",i)] = p[0]/ p[1]
        //         else:
        //             operations[pyformat("f",i)] = lambda x, coefs=p: (coefs[0] - coefs[1] * x) / coefs[2]
        //
        //     fig, ax = plt.subplots()
        //     ax.plot([round(-optimal_point[0]*30), round(optimal_point[0]*30)], [0,0], linewidth=2, color="black", alpha=1.0)
        //     ax.plot([0,0], [round(-optimal_point[1]*30), round(optimal_point[1]*30)], linewidth=2, color="black", alpha=1.0)
        //
        //
        //     for key, func in operations.items():
        //         if type(func) == float:
        //             x_values = np.full(len(domain_animation), func)
        //             // y_values = np.arange(0.0, 10.0, 1.0)
        //             ax.plot(x_values, domain_animation, alpha=0.75, linestyle="--")
        //         else: 
        //             x_values = domain_animation
        //             y_values = [func(x) for x in domain_animation]
        //             ax.plot(x_values, y_values, alpha=0.75, linestyle="--")
        //
        //     line, = ax.plot([])
        //
        //     def animate(i):
        //         // A*x + B*y = Z = 0 --> y = (-A * x) / B + i *
        //         y = (-z[0]*domain_animation) / z[1] + i
        //         line.set_data(domain_animation, y)
        //         return line,
        //
        //     ani = animation.FuncAnimation(
        //         fig, func=animate, frames=np.arange(round(-optimal_point[0]*0.1), 
        //         round(optimal_point[0]*3), 0.1), interval=1)
        //
        //     ax.add_patch(polygon)
        //
        //     ax.plot(optimal_point[0], optimal_point[1], marker="o", markersize=13, 
        //             markeredgecolor="red", markerfacecolor="yellow", 
        //             label=str(optimal_point[0])+", "+str(optimal_point[1]))
        //
        //     ax.set_xlim(round(-optimal_point[0]*2), round(optimal_point[0]*4))
        //     ax.set_ylim(round(-optimal_point[1]*2), round(optimal_point[1]*4))
        //     ax.grid()
        //     ax.legend()
        //
        //     plt.show()
        // }
    }

    pub fn solve(&mut self) {

        self.get_all_intersections();
        self.get_feasible_intersections();
        self.optimize();
        self.prepare_for_graphic();
        self.graphic();
    }
}
