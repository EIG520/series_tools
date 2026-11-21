use series_tools::power::{Constant, Geometric, PowSeries};

fn main() {
    let k = PowSeries::new(Geometric(1.0));
    let c = PowSeries::new(Constant(-1.0, true));
    (k.clone() * k.clone() * k.clone()).print_terms_ln(10);

    (k.clone() + c.clone()).print_terms_ln(10);
    (k.clone().apply_on(k.clone() + c.clone())).print_terms_ln(10);
}