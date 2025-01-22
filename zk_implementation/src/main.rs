use ::std::iter::Sum;
use ::std::ops::Mul;
use ark_ff::PrimeField;
use ark_std::test_rng;
use std::iter::Product;
use std::ops::Add;
#[derive(Clone, Debug, PartialEq)]
struct Polynomials<P: PrimeField> {
    value: Vec<P>,
}

impl<P: PrimeField> Polynomials<P> {
    fn new(_value: Vec<P>) -> Self {
        Polynomials { value: _value }
    }
    fn evaluate(&self, x: P) -> P {
        self.value
            .iter()
            .enumerate()
            .map(|(index, coef)| *coef * x.pow([index as u64]))
            .sum()
    }

    fn degree(&self) -> usize {
        self.value.len() - 1
    }

    fn interpolate(xs: Vec<P>, ys: Vec<P>) -> Polynomials<P> {
        xs.iter()
            .zip(ys.iter())
            .map(|(x, y)| Self::basis(x, &xs).scalar_mul(y))
            .sum()
    }

    fn basis(x: &P, interpolating_set: &[P]) -> Self {
        //numerator
        let numerator: Polynomials<P> = interpolating_set
            .iter()
            .filter(|val| *val != x)
            .map(|x_n| Polynomials::new(vec![-*x_n, P::one()]))
            .product();

        //denominator
        // Changed to use inverse() instead of the normal division
        let denominator: P = numerator.evaluate(*x).inverse().unwrap();
        numerator.scalar_mul(&denominator)
    }

    fn scalar_mul(&self, y: &P) -> Self {
        Polynomials::new(self.value.iter().map(|coeff| *coeff * y).collect())
    }
}

impl<P: PrimeField> Sum for Polynomials<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Polynomials::new(vec![P::zero()]), |result, poly| {
            &result + &poly
        })
    }
}

impl<P: PrimeField> Mul for Polynomials<P> {
    type Output = Polynomials<P>;
    // using for loop but i kind of prefer iterators :(

    // fn mul(self, rhs: Self) -> Polynomials {
    //     let new_deg = self.degree() + rhs.degree();
    //     let mut result = vec![0.0; new_deg + 1];
    //     for i in 0..self.value.len() {
    //         for j in 0..rhs.value.len() {
    //             result[i + j] += self.value[i] * rhs.value[j]
    //         }
    //     }

    //     Polynomials { value: result }
    // }
    fn mul(self, rhs: Self) -> Polynomials<P> {
        let new_deg = self.degree() + rhs.degree();
        let mut result = vec![P::zero(); new_deg + 1];

        self.value
            .iter()
            .enumerate()
            .flat_map(|(i, &a)| {
                rhs.value
                    .iter()
                    .enumerate()
                    .map(move |(j, &b)| (i + j, a * b))
            })
            .for_each(|(idx, val)| result[idx] += val);

        Polynomials::new(result)
        // Polynomials { value: result }
    }
}

impl<P: PrimeField> Product for Polynomials<P> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Polynomials::new(vec![P::one()]), |result, poly| {
            result * poly
        })

        // this works also i just prefer iterators like i said earlier :(
        // let mut result = Polynomials::new(vec![1.0]);
        // for poly in iter {
        //     result = result * poly;
        // }
        // result
    }
}

impl<P: PrimeField> Add for &Polynomials<P> {
    type Output = Polynomials<P>;

    fn add(self, rhs: Self) -> Self::Output {
        // this basically checks the index of the two polynomials and stops when its longer than each other
        // and brings in the rest as single values they lack mates :(
        let (mut bigger, &smaller) = if self.degree() < rhs.degree() {
            (rhs.clone(), &self)
        } else {
            (self.clone(), &rhs)
        };

        bigger
            .value
            .iter_mut()
            .zip(smaller.value.iter())
            .map(|(b_coef, s_coef)| *b_coef += s_coef)
            .for_each(drop);

        Polynomials::new(bigger.value)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use crate::Polynomials;
    use ark_bn254::Fq;
    use ark_ff::PrimeField;
    use std::vec;

    fn poly_1() -> Polynomials<Fq> {
        // f(x) = 1 + 2x + 3x^2
        Polynomials {
            value: vec![Fq::from(1), Fq::from(2), Fq::from(3)],
        }
    }

    fn poly_2() -> Polynomials<Fq> {
        // f(x) = 4x + 3 + 5x^11

        Polynomials {
            value: [
                vec![Fq::from(3), Fq::from(4)],
                vec![Fq::from(0); 9],
                vec![Fq::from(5)],
            ]
            .concat(),
        }
    }

    #[test]
    fn test_degree() {
        let p = Polynomials {
            value: vec![Fq::from(1), Fq::from(2), Fq::from(3)],
        };
        p.degree();

        assert_eq!(p.degree(), 2);
    }

    #[test]
    fn test_eval() {
        let p = Polynomials {
            value: vec![Fq::from(1), Fq::from(2), Fq::from(3)],
        };
        p.evaluate(Fq::from(1));
        assert_eq!(p.evaluate(Fq::from(1)), Fq::from(6));
    }

    #[test]
    fn test_add_polynomials() {
        // f(x) = 1 + 2x + 3x^2
        // +
        // f(x) = 4x + 3 + 5x^11

        // r(x) = 4 + 6x + 3x^2 + 5x^11

        assert_eq!(
            (&poly_1() + &poly_2()).value,
            [
                vec![Fq::from(4), Fq::from(6), Fq::from(3)],
                vec![Fq::from(0); 8],
                vec![Fq::from(5)],
            ]
            .concat()
        );
    }

    #[test]
    fn test_mul() {
        // f(x) = 5 + 2x^2
        let poly_1 = Polynomials {
            value: vec![Fq::from(5), Fq::from(0), Fq::from(2)],
        };
        //  f(x) = 2x + 6
        let poly_2 = Polynomials {
            value: vec![Fq::from(6), Fq::from(2)],
        };

        // r(x) = 30 + 10x + 12x^2 + 4x^3
        assert_eq!(
            (poly_1 * poly_2).value,
            vec![Fq::from(30), Fq::from(10), Fq::from(12), Fq::from(4)]
        );
    }

    #[test]
    fn test_interpolate() {
        // f(x) = 2x
        // [(2,4),(4,3)]

        let m = Polynomials::interpolate(
            vec![Fq::from(2), Fq::from(4)],
            vec![Fq::from(4), Fq::from(8)],
        );
        assert_eq!(m.value, vec![Fq::from(0), Fq::from(2)]);
    }

    #[test]
    fn test_basis_function() {
        // Test basis polynomial for x = 2 with interpolating set [2, 4]
        let x = Fq::from(2);
        let interpolating_set = vec![Fq::from(2), Fq::from(4)];
        let basis = Polynomials::basis(&x, &interpolating_set);

        // L₁(x) should be (-1/2)x + 2
        // In prime field: [-*x_n, P::one()] creates (x - 4)
        // Then we multiply by inverse of (2 - 4) = -2
        println!("Basis for x=2: {:?}", basis.value);
    }

    #[test]
    fn test_scalar_mul() {
        // Test scalar multiplication with a simple polynomial
        let poly = Polynomials::new(vec![Fq::from(1), Fq::from(1)]); // x + 1
        let scalar = Fq::from(2);
        let result = poly.scalar_mul(&scalar);
        println!("Scalar mul result: {:?}", result.value);
    }
}
