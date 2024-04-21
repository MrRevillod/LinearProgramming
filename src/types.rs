
use std::collections::HashMap;

#[allow(dead_code)]

pub type A = Vec<Vec<f64>>;
pub type B = Vec<f64>;
pub type C = Vec<f64>;
pub type Z = Vec<f64>;

pub type Operations = Vec<Operation>;
pub type Intersections = Vec<Point>;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

#[allow(warnings)]
#[derive(Debug, Clone)]
pub enum Operation {
    Lt,
    Gt,
    Eq,
}

#[derive(Debug, Clone)]
pub enum Algorithm {
    Graphic(GraphicMethod),
    Simplex(SimplexMethod),
}

#[derive(Debug, Clone)]
pub enum ProblemKind {
    Maximize,
    Minimize,
}

#[derive(Debug, Clone)]
pub struct GraphicMethod {
    pub kind: ProblemKind,
    pub a: A,
    pub b: B,
    pub z: Z,
    pub operations: Operations,
    pub intersections: Intersections,
    pub python_intersections: Vec<Vec<f64>>,
    pub optimal_point: [f64; 2],
    pub utility: f64,
    pub inequalities: Vec<Vec<f64>>,
}

#[allow(warnings)]
#[derive(Debug, Clone)]
pub struct SimplexMethod {
    pub kind: ProblemKind,
    pub a: A,
    pub b: B,
    pub c: C,
    pub operations: Operations,
    pub increased: Vec<Vec<f64>>,
    pub table: Vec<Vec<String>>,
    pub pivot: (usize, usize),
    pub two_fases: bool,
    pub n_vars: usize,
    pub var_positions: HashMap<char, Vec<usize>>,
    pub artificial_rows: Vec<usize>,
}
