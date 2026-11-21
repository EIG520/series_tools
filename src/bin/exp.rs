use series_tools::power::{Constant, Exp, PowSeries};

fn main() {
    // e^x
    let k = PowSeries::new(Exp::new());
    // e^x - 1
    let c = k.clone() + PowSeries::new(Constant(-1.0, true));

    (k.clone()).print_terms_ln(10);
    (c.clone()).print_terms_ln(10);

    // e^(e^x-1)
    (k.clone().apply_on(c.clone())).print_terms_ln(10);
}
