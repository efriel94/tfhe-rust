use crate::torus::torus::{TorusElement};

use std::ops::{Add, Neg, Sub};

impl Add for TorusElement {
    type Output = TorusElement;

    fn add(self, other: TorusElement) -> Self::Output {
        let result = (self.value + other.value).rem_euclid(1.0); 
        TorusElement { value: (result) }
    }
}

impl Neg for TorusElement {
    type Output = TorusElement;

    fn neg(self) -> Self::Output {
        TorusElement { value: (-self.value).rem_euclid(1.0) }
    }
}

impl Sub for TorusElement {
    type Output = TorusElement;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}
// testing the torus (R/Z, +) albelian group axioms 
#[cfg(test)]
mod tests {
    use super::*;

    //closure under the group operation i.e addition
    #[test]
    fn test_closure() {
        let t1 = TorusElement { value: 0.45 };
        let t2 = TorusElement { value: 0.95 };
        let result = t1 + t2;

        assert!(
            (result.value < 1.0) && (result.value > 0.0),
            "Closure has failed: lhs={:?}, rhs={:?}, result={:?}",
            t1,
            t2,
            result
        )
    }

    //associativity 
    #[test]
    fn test_associativity() {

        let t1 = TorusElement { value: 0.25 };
        let t2 = TorusElement { value: 0.85 };
        let t3 = TorusElement { value: 0.55 };

        // (a + b) + c == a + (b + c)
        let result1 = (t1 + t2) + t3;
        let result2 = t1 + (t2 + t3);
        println!("result1 : {:?}", result1);
        println!("result2 : {:?}", result2);
        assert!(
            (result1.value - result2.value).abs() < 1e-6, //floating point error
            "Associativity failed: lhs={:?} rhs={:?}",
            result1,
            result2,
        )  
    }

    //identity element: a + e = e + a
    #[test]
    fn test_identity_element() {

        let t1 = TorusElement { value: 0.567 };
        let e = TorusElement { value: 0.0 };

        //a + e = e + a = a
        let result1 = t1 + e;
        let result2 = e + t1;
        assert!(
            result1 == result2,
            "Identity element failed: lhs={:?}, rhs={:?}",
            result1,
            result2
        )

    }

    //inverses
    #[test]
    fn test_group_inverse() {

        let t1 = TorusElement { value: 0.55 };
        let inverse_t1 = -t1; // -0.55 mod 1 = 0.45

        assert!(
            inverse_t1.value == 0.45,
            "Inverses failed: lhs={:?}, rhs={:?}",
            t1,
            inverse_t1
        );
    }

    //commutativity: a + b = b + a
    #[test]
    fn test_commutativity() {

        let t1 = TorusElement { value: 0.55 };
        let t2 = TorusElement { value: 0.35 };

        let result1 = t1 + t2;
        let result2 = t2 + t1;

        assert!(
            result1==result2,
            "Commutativity failed: lhs={:?}, rhs={:?}",
            result1,
            result2
        )
    }
}