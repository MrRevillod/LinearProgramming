
// función .iter() | .iter_mut()
//
// Se puede usar en un for value in vector
// permite iterar sobre un vector/matriz y obtener una referencia
// mutable o inmutable, para usar ese valor debe ser dereferenciado usando el *
//
// Ejemplo: Cambiar las filas de una matrix por vectores vacios
//
// for row in self.matrix.iter_mut() {
//      *row = Vec::new();
// }

// función .enumerate() 
//
// Permite obtener un iterable i (contador) y un valor como 
// en el pasado ejemplo. Debe usarse en conjunto con 
// .iter() | iter_mut() dependiendo el proposito
//
// Ejemplo: Si 2 vectores tienen la misma len() y queremos reasignar valores entre sí
//
// let a = vec![1, 2, 3]
// let b = vec![2, 3, 4]
//
// for (i, value) in a.iter_mut().enumerate() {
//      *value = b[i];
// }

// Enums
//
// Una enum es una forma de agrupar varios conceptos/elementos
// que comparten una misma caracteristica.
//
// Ejemplo Enum ROL, almacena valores, Administrador y usuario.
//
// Ejemplo Enum Algoritmo, almacena Metodo Grafico y Metodo simplex
//
// La enum permite retornar o manejar un Tipo que engloba o representa
// a un conjunto de tipos.
