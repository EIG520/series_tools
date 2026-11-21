// For handling power series

use std::ops::{Add, Mul};

use crate::partition::PartitionGenerator;

pub type Num = f32;
pub trait Coeff: Iterator<Item = Num> + Clone {}

impl<T: Coeff> Coeff for PowSeries<T> {}
impl<T: Coeff> Coeff for PowSeriesK<T> {}
impl<T: Coeff, T2: Coeff> Coeff for PowSum<T, T2> {}
impl<T: Coeff, T2: Coeff> Coeff for PowProduct<T, T2> {}
impl<T: Coeff, T2: Coeff> Coeff for PowNest<T, T2> {}
impl Coeff for Geometric {}
impl Coeff for Constant {}
impl Coeff for Exp {}

#[derive(Clone)]
pub struct PowSeries<T: Coeff> {
    coeffs: T,
}

impl<T: Coeff> PowSeries<T> {
    pub fn new(f: T) -> PowSeries<T> {
        PowSeries { coeffs: f }
    }
}

impl<T: Coeff> Iterator for PowSeries<T> {
    type Item = Num;

    fn next(&mut self) -> Option<Self::Item> {
        self.coeffs.next()
    }
}

impl<T: Coeff> PowSeries<T> {
    pub fn print_terms(&self, n: usize) {
        let mut t = self.clone().take(n);

        for i in 0..n {
            print!("{}x^{i} + ", t.next().unwrap_or_default())
        }

        print!("...");
    }

    pub fn print_terms_ln(&self, n: usize) {
        self.print_terms(n);
        println!();
    }
}

impl<T: Coeff, T2: Coeff> Add<PowSeries<T2>> for PowSeries<T> {
    type Output = PowSeries<PowSum<T, T2>>;

    fn add(self, rhs: PowSeries<T2>) -> Self::Output {
        PowSeries {
            coeffs: PowSum {
                factor1: self,
                factor2: rhs,
            },
        }
    }
}

#[derive(Clone)]
pub struct PowSum<T: Coeff, T2: Coeff> {
    factor1: PowSeries<T>,
    factor2: PowSeries<T2>,
}

impl<T: Coeff, T2: Coeff> Iterator for PowSum<T, T2> {
    type Item = Num;
    fn next(&mut self) -> Option<Self::Item> {
        Some(
            self.factor1.coeffs.next().unwrap_or_default()
                + self.factor2.coeffs.next().unwrap_or_default(),
        )
    }
}

impl<T: Coeff, T2: Coeff> Mul<PowSeries<T2>> for PowSeries<T> {
    type Output = PowSeries<PowProduct<T, T2>>;

    fn mul(self, rhs: PowSeries<T2>) -> Self::Output {
        PowSeries {
            coeffs: PowProduct {
                factor1: self,
                factor2: rhs,
                pow: 0,
            },
        }
    }
}

#[derive(Clone)]
pub struct PowProduct<T: Coeff, T2: Coeff> {
    factor1: PowSeries<T>,
    factor2: PowSeries<T2>,
    pow: usize,
}

impl<T: Coeff, T2: Coeff> Iterator for PowProduct<T, T2> {
    type Item = Num;
    fn next(&mut self) -> Option<Self::Item> {
        self.pow += 1;

        let r = self
            .factor1
            .clone()
            .coeffs
            .take(self.pow)
            .zip(
                self.factor2
                    .clone()
                    .coeffs
                    .take(self.pow)
                    .collect::<Vec<Num>>()
                    .iter()
                    .rev(),
            )
            .map(|(a, &b)| a * b)
            .sum();
        Some(r)
    }
}

impl<T: Coeff> Mul<Num> for PowSeries<T> {
    type Output = PowSeries<PowSeriesK<T>>;
    fn mul(self, rhs: Num) -> Self::Output {
        PowSeries {
            coeffs: PowSeriesK { ser: self, n: rhs },
        }
    }
}

#[derive(Clone)]
pub struct PowSeriesK<T: Coeff> {
    ser: PowSeries<T>,
    n: Num,
}

impl<T: Coeff> Iterator for PowSeriesK<T> {
    type Item = Num;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.ser.next().unwrap_or_default() * self.n)
    }
}

// Requires that factor2 (rhs) has no constant term (!!!)
impl<T: Coeff> PowSeries<T> {
    pub fn apply_on<T2: Coeff>(self, rhs: PowSeries<T2>) -> PowSeries<PowNest<T, T2>> {
        PowSeries {
            coeffs: PowNest {
                factor1: self,
                factor2: rhs,
                pow: 0,
            },
        }
    }
}

#[derive(Clone)]
pub struct PowNest<T: Coeff, T2: Coeff> {
    factor1: PowSeries<T>,
    factor2: PowSeries<T2>,
    pow: usize,
}

impl<T: Coeff, T2: Coeff> Iterator for PowNest<T, T2> {
    type Item = Num;
    fn next(&mut self) -> Option<Self::Item> {
        self.pow += 1;

        let mut c = self.factor1.clone().coeffs.take(self.pow);
        let d = self
            .factor2
            .clone()
            .coeffs
            .take(self.pow)
            .collect::<Vec<Num>>();

        let mut s = 0.0; //c.next().unwrap_or_default();

        for i in 0..self.pow {
            let pgen = PartitionGenerator::new(self.pow - 1, i);

            s += pgen
                .map(|a| a.iter().map(|&n| d[n]).product::<Num>())
                .sum::<Num>()
                * c.next().unwrap_or_default();
        }

        Some(s)
    }
}

#[derive(Clone, Copy)]
pub struct Geometric(pub Num);
impl Iterator for Geometric {
    type Item = Num;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0)
    }
}

#[derive(Clone, Copy)]
pub struct Constant(pub Num, pub bool);
impl Iterator for Constant {
    type Item = Num;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 {
            self.1 = false;
            Some(self.0)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct Exp {
    cur: usize,
    curk: Num,
}

impl Iterator for Exp {
    type Item = Num;

    fn next(&mut self) -> Option<Self::Item> {
        let k = self.curk;
        self.cur += 1;
        self.curk /= self.cur as Num;
        Some(k)
    }
}

impl Exp {
    pub fn new() -> Self {
        Self { cur: 0, curk: 1.0 }
    }
}
