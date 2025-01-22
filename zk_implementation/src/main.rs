use ::std::iter::Sum;
use ::std::ops::Mul;
use std::iter::Product;
use std::ops::Add;
use std::vec;
#[derive(Clone, Debug, PartialEq)]
struct Polynomials {
    value: Vec<f64>,
}

impl Polynomials {
    fn new(_value: Vec<f64>) -> Self {
        Polynomials { value: _value }
    }
    fn evaluate(&self, x: f64) -> f64 {
        self.value
            .iter()
            .enumerate()
            .map(|(index, coef)| coef * x.powf(index as f64))
            .sum()
    }
    fn degree(&self) -> usize {
        self.value.len() - 1
    }

    fn interpolate(xs: Vec<f64>, ys: Vec<f64>) -> Polynomials {
        xs.iter()
            .zip(ys.iter())
            .map(|(x, y)| Self::basis(x, &xs).scalar_mul(y))
            .sum()
    }

    fn basis(x: &f64, interpolating_set: &[f64]) -> Self {
        //numerator
        let numerator: Polynomials = interpolating_set
            .iter()
            .filter(|val| *val != x)
            .map(|x_n| Polynomials::new(vec![-x_n, 1.0]))
            .product();

        //denominator
        let denominator = 1.0 / numerator.evaluate(*x);
        numerator.scalar_mul(&denominator)
    }

    fn scalar_mul(&self, y: &f64) -> Self {
        Polynomials::new(self.value.iter().map(|coeff| coeff * y).collect())
    }
}

impl Sum for Polynomials {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Polynomials::new(vec![0.0]), |result, poly| &result + &poly)
    }
}

impl Mul for Polynomials {
    type Output = Polynomials;
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
    fn mul(self, rhs: Self) -> Polynomials {
        let new_deg = self.degree() + rhs.degree();
        let mut result = vec![0.0; new_deg + 1];

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

impl Product for Polynomials {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Polynomials::new(vec![1.0]), |result, poly| result * poly)

        // this works also i just prefer iterators like i said earlier :(
        // let mut result = Polynomials::new(vec![1.0]);
        // for poly in iter {
        //     result = result * poly;
        // }
        // result
    }
}

impl Add for &Polynomials {
    type Output = Polynomials;

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
    let p = Polynomials {
        value: vec![1.0, 2.0, 3.0],
    };
    p.degree();

    p.evaluate(1.0);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::Polynomials;

    fn poly_1() -> Polynomials {
        // f(x) = 1 + 2x + 3x^2
        Polynomials {
            value: vec![1.0, 2.0, 3.0],
        }
    }

    fn poly_2() -> Polynomials {
        // f(x) = 4x + 3 + 5x^11

        Polynomials {
            value: [vec![3.0, 4.0], vec![0.0; 9], vec![5.0]].concat(),
        }
    }

    #[test]
    fn test_degree() {
        let p = Polynomials {
            value: vec![1.0, 2.0, 3.0],
        };
        p.degree();

        assert_eq!(p.degree(), 2);
    }

    #[test]
    fn test_eval() {
        let p = Polynomials {
            value: vec![1.0, 2.0, 3.0],
        };
        p.evaluate(1.0);
        assert_eq!(p.evaluate(1.0), 6.0);
    }

    #[test]
    fn test_add_polynomials() {
        // f(x) = 1 + 2x + 3x^2
        // +
        // f(x) = 4x + 3 + 5x^11

        // r(x) = 4 + 6x + 3x^2 + 5x^11

        assert_eq!(
            (&poly_1() + &poly_2()).value,
            [vec![4.0, 6.0, 3.0], vec![0.0; 8], vec![5.0]].concat()
        );
    }

    #[test]
    fn test_mul() {
        // f(x) = 5 + 2x^2
        let poly_1 = Polynomials {
            value: vec![5.0, 0.0, 2.0],
        };
        //  f(x) = 2x + 6
        let poly_2 = Polynomials {
            value: vec![6.0, 2.0],
        };

        // r(x) = 30 + 10x + 12x^2 + 4x^3
        assert_eq!((poly_1 * poly_2).value, vec![30.0, 10.0, 12.0, 4.0]);
    }

    #[test]
    fn test_interpolate() {
        // f(x) = 2x
        // [(2,4),(4,3)]

        let m = Polynomials::interpolate(vec![2.0, 4.0], vec![4.0, 8.0]);
        assert_eq!(m.value, vec![0.0, 2.0]);
    }
}
