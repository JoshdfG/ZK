use ::std::iter::Sum;
use ::std::ops::Mul;
use ark_ff::PrimeField;
use std::iter::Product;
use std::ops::Add;
#[derive(Clone, Debug, PartialEq)]
pub struct Polynomials<P: PrimeField> {
    pub value: Vec<P>,
}

impl<P: PrimeField> Polynomials<P> {
    pub fn new(_value: Vec<P>) -> Self {
        Polynomials { value: _value }
    }
    pub fn evaluate(&self, x: P) -> P {
        self.value
            .iter()
            .enumerate()
            .map(|(index, coef)| *coef * x.pow([index as u64]))
            .sum()
    }

    pub fn degree(&self) -> usize {
        self.value.len() - 1
    }

    pub fn interpolate(xs: Vec<P>, ys: Vec<P>) -> Polynomials<P> {
        xs.iter()
            .zip(ys.iter())
            .map(|(x, y)| Self::basis(x, &xs).scalar_mul(y))
            .sum()
    }

    pub fn basis(x: &P, interpolating_set: &[P]) -> Self {
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

    pub fn scalar_mul(&self, y: &P) -> Self {
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
