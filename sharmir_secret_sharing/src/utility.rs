use ::std::iter::Sum;
use ::std::ops::Mul;
use ark_ff::PrimeField;
use std::iter::Product;
use std::ops::Add;
#[derive(Clone, Debug, PartialEq)]
pub struct DenseUnivariatePolynomial<P: PrimeField> {
    pub coefficients: Vec<P>,
}

impl<P: PrimeField> DenseUnivariatePolynomial<P> {
    pub fn new(_value: Vec<P>) -> Self {
        DenseUnivariatePolynomial { coefficients: _value }
    }
    pub fn evaluate(&self, x: P) -> P {
        self.coefficients
            .iter()
            .enumerate()
            .map(|(index, coef)| *coef * x.pow([index as u64]))
            .sum()
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn interpolate(xs: Vec<P>, ys: Vec<P>) -> DenseUnivariatePolynomial<P> {
        xs.iter()
            .zip(ys.iter())
            .map(|(x, y)| Self::basis(x, &xs).scalar_mul(y))
            .sum()
    }

    pub fn basis(x: &P, interpolating_set: &[P]) -> Self {
        let numerator: DenseUnivariatePolynomial<P> = interpolating_set
            .iter()
            .filter(|val| *val != x)
            .map(|x_n| DenseUnivariatePolynomial::new(vec![-*x_n, P::one()]))
            .product();
        let denominator: P = numerator.evaluate(*x).inverse().unwrap();
        numerator.scalar_mul(&denominator)
    }

    pub fn scalar_mul(&self, y: &P) -> Self {
        DenseUnivariatePolynomial::new(self.coefficients.iter().map(|coeff| *coeff * y).collect())
    }
}

impl<P: PrimeField> Sum for DenseUnivariatePolynomial<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(DenseUnivariatePolynomial::new(vec![P::zero()]), |result, poly| {
            &result + &poly
        })
    }
}

impl<P: PrimeField> Mul for DenseUnivariatePolynomial<P> {
    type Output = DenseUnivariatePolynomial<P>;
    fn mul(self, rhs: Self) -> DenseUnivariatePolynomial<P> {
        let new_deg = self.degree() + rhs.degree();
        let mut result = vec![P::zero(); new_deg + 1];

        self.coefficients
            .iter()
            .enumerate()
            .flat_map(|(i, &a)| {
                rhs.coefficients
                    .iter()
                    .enumerate()
                    .map(move |(j, &b)| (i + j, a * b))
            })
            .for_each(|(idx, val)| result[idx] += val);

        DenseUnivariatePolynomial::new(result)
    }
}

impl<P: PrimeField> Product for DenseUnivariatePolynomial<P> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(DenseUnivariatePolynomial::new(vec![P::one()]), |result, poly| {
            result * poly
        })
    }
}

impl<P: PrimeField> Add for &DenseUnivariatePolynomial<P> {
    type Output = DenseUnivariatePolynomial<P>;

    fn add(self, rhs: Self) -> Self::Output {
        // this basically checks the index of the two polynomials and stops when its longer than each other
        // and brings in the rest as single values they lack mates :(
        let (mut bigger, &smaller) = if self.degree() < rhs.degree() {
            (rhs.clone(), &self)
        } else {
            (self.clone(), &rhs)
        };

        bigger
            .coefficients
            .iter_mut()
            .zip(smaller.coefficients.iter())
            .map(|(b_coef, s_coef)| *b_coef += s_coef)
            .for_each(drop);

        DenseUnivariatePolynomial::new(bigger.coefficients)
    }
}