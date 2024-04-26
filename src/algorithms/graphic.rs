
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

    // Obtener todas las intersecciones del sistema. Para lograr esto 
    // se deben generar todas las combinaciones entre las inecuaciones del problema

    pub fn get_all_intersections(&mut self) {

        let mut intersections = Intersections::new();

        for i in 0..self.a.len() {

            for j in i + 1..self.a.len() {

                // Tomar un par de coeficientes y resultados
                // y calcular su intersección mediante el metodo de cramer

                let pair_of_eq = vec![
                    self.a[i].clone(),
                    self.a[j].clone()
                ];

                let pair_of_res = vec![self.b[i], self.b[j]];
                let intersection = cramer(&pair_of_eq, &pair_of_res);

                // Si el metodo de cramer retorna un error por un determinante igual a 0, 
                // puede significar que no hay una solución entre ambas ecuaciones

                match intersection {
                    Ok(point) => intersections.push(point),
                    Err(_) => continue
                }
            }
        }

        self.intersections = intersections;
    }

    // Una vez se obtienen todas las posibles intersecciones en
    // el problema, se deben filtrar entre las que cumplen con las restricciones

    pub fn get_feasible_intersections(&mut self) {

        let mut intesections = Intersections::new();

        // Se crea una etiqueta posicional para indicar un punto de retorno
        // en el código, iteramos sobre las intersecciones del problema

        'outer: for Point { x, y } in self.intersections.iter() {

            for i in 0..self.a.len() {

                // Para validar estas intersecciones se debe probar cada par de puntos (x, y)
                // en cada combinación de inecuaciones, dependiendo de su operador (>= | <=)
                // se debe usar un operador u otro

                let is_valid = match self.operations[i] {
                    Operation::Gt => self.a[i][0] * x + self.a[i][1] * y >= self.b[i],
                    Operation::Lt => self.a[i][0] * x + self.a[i][1] * y <= self.b[i],
                    Operation::Eq => self.a[i][0] * x + self.a[i][1] * y == self.b[i],
                };

                // is_valid indica si la presente interseccion (x, y) es valida o no,
                // si no lo es es innecesario seguir intentando ya que ya que
                // este punto no corresponderá a la región objetivo

                if !is_valid {
                    continue 'outer
                }
            }

            intesections.push(Point::new(x.clone(), y.clone()))
        }

        self.intersections = intesections;
    }

    // Función de optimización, dependiendo del tipo de problema
    // se debe usar un caso de optimización o no.

    pub fn optimize(&mut self) {

        let optimize = match self.kind {
            ProblemKind::Minimize => |a: f64, b: f64| a < b,
            ProblemKind::Maximize => |a: f64, b: f64| a > b,
        };

        // Se recorren los puntos filtrados de intersección
        // a este punto todos estos puntos son parte de la región factible

        for Point { x, y } in self.intersections.iter() {

            // Se calcula la utilidad utilizando los coheficiente
            // de la función objetivo y se evalua la optimización
            // y el punto óptimo

            let utility = self.z[0] * x + self.z[1] * y;

            if optimize(utility, self.utility) {
                self.utility = utility;
                self.optimal_point = [x.clone(), y.clone()];
            }
        }
    }
    
    // Preparar el problema para graficarlo

    pub fn prepare_for_graphic(&mut self) {

        // Preparar vector con ecuaciones de la forma [B, c_x, c_y]
        // con el fin de generar las rectas en el gráfico de python

        for i in 0..self.a.len() {
            self.inequalities.push(
                vec![self.b[i].clone(), self.a[i][0].clone(), self.a[i][1].clone()]
            )
        }
        
        let mut points = Vec::new();
        let mut intersections = self.intersections.clone();

        // Formula de la distancia entre 2 puntos, mediante este metodo
        // podemos ordenar un vector de puntos x, y siguiendo un orden lógico
        // mediante su distancia y así crear un poligono

        let dist = |uno: &Point, dos: &Point| {
            ((dos.x - uno.x).powf(2f64) + (dos.y - uno.y).powf(2f64)).sqrt()
        };

        // Obtener un punto de partica inicial y añadirlo a un nuevo vector ordenado

        let mut current_point = intersections.remove(0);
        points.push(vec![current_point.x.clone(), current_point.y.clone()]);

        // Ordenar los puntos mientras no se vacie el vector de intersecciones

        while !intersections.is_empty() {

            // Aplicar función sort by para ordenar 2 elementos mediante una comparación numerica

            intersections.sort_by(
                |a, b| dist(&current_point, a).partial_cmp(&dist(&current_point, b)).unwrap()
            );

            current_point = intersections.remove(0);
            points.push(vec![current_point.x.clone(), current_point.y.clone()]);
        }

        self.python_intersections = points;

        // Despreciar restricciones de no negatividad para no graficarlas

        self.inequalities.pop();
        self.inequalities.pop();
    }

    #[allow(warnings)]
    pub fn graphic(&self) {

        let intersections = self.python_intersections.clone();
        let inequalities = self.inequalities.clone();
        let optimal_point = self.optimal_point.clone();
        let z = self.z.clone();
        let b = self.b.clone();

        // python! {
        // 
        //     // matplotlib.pyplot -> Graficos
        //     import matplotlib.pyplot as plt
        //     // matplotlib.patches -> Rellenar el área factible con Polygon
        //     from matplotlib.patches import Polygon
        //     // numpy -> Manipulación de arreglos para los datos de los gráficos
        //     import numpy as np
        // 
        //     // Pasamos de objetos en rust a python
        //     intersections = 'intersections
        //     inequalities = 'inequalities
        //     optimal_point = 'optimal_point
        //     b = 'b
        //     z = 'z
        // 
        //     // Arreglo de valores que se adaptan segun el mayor valor del vector 
        //     // b(terminos independientes de las restricciones)
        // 
        //     domain = np.arange(-max(b)*1.25, max(b)*2)
        // 
        //     // Diccionario para guardar las funciones de las restricciones y Z
        //     operations = {}
        // 
        //     // Función de formateo de string para añadir las funciones de manera dinamica al diccionario
        //     def pyformat(x, i):
        //         return x+str(i)
        // 
        //     // Rellenar de forma dinamica el diccionario con las restricciones
        //     for i, p in enumerate(inequalities):
        //         if p[2] == 0:
        //             operations[pyformat("x",i)] = p[0]/ p[1]
        //         else:
        //             operations[pyformat("f",i)] = lambda x, coefs=p: (coefs[0] - coefs[1] * x) / coefs[2]
        // 
        //     // Inicialización de los objetos de graficación(no se utiliza el objeto fig, que sirve para modificar la "ventana")
        //     fig, ax = plt.subplots()
        // 
        //     // Graficación de los ejes x ,eje y. Proporcinalmente al Par coordenado Óptimo
        //     ax.plot([round(-max(b)*10), round(max(b)*10)], [0,0], color="black", alpha=0.3)
        //     ax.plot([0,0], [round(-max(b)*10), round(max(b)*10)], color="black", alpha=0.3)
        // 
        //     // Se recorre el diccionario de restricciones
        //     for key, func in operations.items():
        //         // si es constante no se evalua la función, si no que el recorrido se rellena con el mismo valor
        //         if type(func) == float:
        //             x_values = np.full(len(domain), func)
        //             ax.plot(x_values, domain, alpha=0.4, linestyle="--")
        //         // en los otros casos, se evalua las funciones con el dominio anteriormente generado
        //         else: 
        //             x_values = domain
        //             y_values = [func(x) for x in domain]
        //             ax.plot(x_values, y_values, alpha=0.4, linestyle="--")
        // 
        //     // Polygon es una clase que recibe una matriz que contiene las intersecciones, y forma un polygono que con
        //     // el parametro "fill" se rellena
        //     polygon = Polygon(intersections, closed=False, fill=True, color="red", alpha=0.3)
        // 
        //     // Pinta el poligono
        //     ax.add_patch(polygon)
        // 
        //     // Marcar las intersecciones de la región factible con un punto y poner su punto P(x,y) = $(z)
        //     for x, y in intersections:
        //         ax.text(x, y, "Z("+str(round(x))+","+str(round(y))+")="+str(round((z[0]*x + z[1]*y),1)),
        //                 ha="center", va="bottom")
        //         ax.plot(x, y, marker="o", markersize=6, color="green", alpha=0.7)
        // 
        //     // El termino inpendiente para poder graficar la función Z sobre el punto óptimo
        //     z_independent_term = optimal_point[1] - (-(z[0]/z[1]) * optimal_point[0])
        // 
        //     // Agregar la función Z ya sobre el punto optimo
        //     operations["z(x)"] = lambda x: -(z[0]/z[1]) * x + z_independent_term
        // 
        //     // Evaluación de la Z para poder graficarla
        //     z_recorrido = [operations["z(x)"](x) for x in domain]
        //     
        //     // Graficar Z
        //     ax.plot(domain, z_recorrido, color="purple", alpha=0.4)
        // 
        //     // Marcar el punto óptimo de forma más marcada
        //     ax.plot(optimal_point[0], optimal_point[1], marker="o", markersize=13, 
        //             markeredgecolor="red", markerfacecolor="yellow")
        // 
        //     // Define los limites a los que llegan los ejes, en base al punto óptimo
        //     ax.set_xlim(round(-max(b)), round(max(b)))
        //     ax.set_ylim(round(-max(b)), round(max(b)))
        //     ax.grid(alpha=0.3)
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
