
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

#[derive(Debug)]
pub enum Operation {
    Lt,
    Gt,
    Eq,
}

#[derive(Debug)]
pub enum ProblemKind {
    Maximize,
    Minimize,
}

#[derive(Debug)]
pub struct GraphicMethod {
    pub kind: ProblemKind,
    pub a: A,
    pub b: B,
    pub z: Z,
    pub operations: Operations,
    pub intersections: Intersections,
    pub optimal_point: Point,
    pub utility: f64,
}

#[derive(Debug)]
pub struct SimplexMethod {
    pub kind: ProblemKind,
    pub a: A,
    pub b: B,
    pub c: C,
    pub operations: Operations,
    pub increased: Vec<Vec<f64>>,
    pub table: Vec<Vec<String>>,
    pub pivot: (usize, usize),
}
