
use regex::Regex;
use lazy_static::lazy_static;

lazy_static!(
    pub static ref ALGORITHM_KIND_REGEX: Regex = Regex::new(r"(Graphic|Simplex)\s*([+-])?").unwrap();
    pub static ref Z_FUNCTION_REGEX: Regex =  Regex::new(r"z =").unwrap();
    pub static ref COEFFS_REGEX: Regex = Regex::new(r"([+-]?\s*\d+(\.\d+)?)\s*x\d+").unwrap();
    pub static ref RESULT_SIDE_REGEX: Regex = Regex::new(r"(<=|>=|=)\s*(-?\d*\.?\d+)").unwrap();
);
