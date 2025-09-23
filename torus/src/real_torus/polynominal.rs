use crate::real_torus::core::TorusElement;
use rand::Rng;

// Torus Polynominals struct
// Considering the polynomial rings RN [X] := R[X]/(X^N +1) and ZN [X] := Z[X]/(X^N + 1), this defines the ZN [X]-module
// TN [X] := RN [X]/ZN [X] = T[X]/(X^N + 1)
// Elements of TN [X] can therefore be seen as polynomials modulo XN + 1 with coefficients in T
#[derive(Debug)]
pub struct TorusPolynominal {
    pub n: usize,                     //degree
    pub m: usize,                     //cyclotomic index (X^N + 1)
    pub coeffs: Vec<TorusElement>     //set of torus elements
}

impl TorusPolynominal {

    // constructing a zero polynominal of degree n
    pub fn zero(degree: usize, cyclotomic_index: usize) -> Self {
        Self { 
            n: degree, 
            m: cyclotomic_index, 
            coeffs: vec![TorusElement {value: 0.0}; degree],
        }
    }

    // constructing a new polynominal from an existing vector
    pub fn from_coeffs(input_coeffs: Vec<TorusElement>, cyclotomic_index: usize) -> Self {
        let n = input_coeffs.len();
        Self { n: n, m: cyclotomic_index, coeffs: input_coeffs }
    }

    //construct a random torus polynominal of order n and cyclotomic order m
    pub fn new_random(degree: usize, cyclotomic_index: usize) -> Self {
        let mut rng = rand::rng();
        let set: Vec<TorusElement> = (0..degree)
            .map(|_| TorusElement { value:  rng.random_range(0.0..1.0)})
            .collect();

        Self { n: degree, m: cyclotomic_index, coeffs: set }
    }
}

