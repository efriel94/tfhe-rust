use crate::torus::torus::{TorusElement};
use std::ops::{Add, Mul, Neg, Sub};

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


//external product operation between integer k and torus element t
//t * k
// if k < 0, then -t * -k
impl Mul<i64> for TorusElement
{
    type Output = TorusElement;

    fn mul(self, rhs: i64) -> Self::Output {

        if rhs < 0 {
            let minus_k = (-rhs) as f32;
            let result = ((-self.value) * (minus_k)).rem_euclid(1.0);
            TorusElement { value: result }
        } else {
            let result = (self.value * rhs as f32).rem_euclid(1.0);
            TorusElement { value: result }
        }
    }
}

//k * t
//if k < 0, -k * -t
impl Mul<TorusElement> for i64
{
    type Output = TorusElement;

    fn mul(self, rhs: TorusElement) -> Self::Output {

        if self < 0 { 
            // -k * -t
            let minus_k = (-self) as f32;
            let result = (minus_k * (-rhs.value)).rem_euclid(1.0);
            TorusElement { value: result }
        } else {
            // k * t
            let result = (self as f32 * (rhs.value)).rem_euclid(1.0);
            TorusElement { value: result }
        }
    }
}


// Testing the torus (R/Z, +) abelian group axioms and its underlying Z-module axioms
// There is a bijective correspondence from Abelian groups (our torus) to Z-modules and vice versa so we must test these axioms are obeyed in
// practice as they form the foundations of the scheme.
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
    fn test_torus_group_commutativity() {

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

    //test external product rhs scalar i.e k * t, repeated addition
    #[test]
    fn test_external_product() {

        let t1 = TorusElement { value : 0.55 };
        let k = 4;
        let result = t1 * k;

        assert!(
            result == (t1 + t1 + t1 + t1),
            "External product failed: lhs={:?}, rhs={}, result{:?}",
            t1,
            k,
            result
        )
    }

    //test external product rhs negative scalar where k<0 therefore -t * -k
    #[test]
    fn test_external_product_minus_scalar() {
        
        let t1 = TorusElement { value : 0.55 };
        let k = 4;
        let result = t1 * k;

        assert!(
            result == (t1 + t1 + t1 + t1),
            "External product failed: lhs={:?}, rhs={}, result{:?}",
            t1,
            k,
            result
        )
    }

    //test external product 
    //if Z \in {0,1}, we have have 0 * t = 0 \in \mathbb{T}
    //if 1, we have 1 * t = t \in \mathbb{T}
    #[test]
    fn test_external_product_zero_scalar() {
        
        let t1 = TorusElement { value: 0.5 };
        let z = 0 as f32;
        let result = (z * t1.value).rem_euclid(1.0);
        let expected = TorusElement { value: 0.0 };

        assert!(
            result == expected.value,
            "External product zero scalar failed: lhs={}, rhs={}, result={}",
            z,
            t1.value,
            result
        )
    }

    //test external product 
    //if Z \in {0,1}, we have have 1 * t = 1 \in \mathbb{T}
    #[test]
    fn test_external_product_one_scalar() {
        todo!()
    }

    //Abelian groups are endowed with a Z-module structure i.e allowing scalar multiplication
    //Scalar multiplication is distributed over the modules
    //For any k,l \in \mathbb{Z} and a,b \in \mathbb{T} we have (k + l) * a = k * a + l * a , and
    // k * (a + b) = k * b + k * b , and
    #[test]
    fn test_external_product_distributivity_over_group_addition() {
        todo!()
    }

    //Testing external product : k * (a + b) = k * b + k * b
    #[test]
    fn test_external_product_distributivity_over_scalar_addition(){
        todo!()
    }

    //Testing external product homomogenous property ie k * (l * a) = (k * l) * a 




}