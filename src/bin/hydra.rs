use std::f32::consts::PI;

use series_tools::power::{Cosine, PowSeries};

pub fn main() {
    let k = (PowSeries::new(Cosine::new()).apply_on(PowSeries::new(vec![0.0,PI].into_iter())) + PowSeries::new(vec![-1.0].into_iter())) * 0.25 + PowSeries::new(vec![0.0,1.5].into_iter());
    k.print_terms_ln(10);

    let s = k.schroder();
    s.print_terms_ln(25);
}
