extern crate nalgebra as na;
extern crate nalgebra_lapack as nl;

use na::{DMatrix, DVector};
use nl::LU;

fn main() {
    let n = 10 as usize;
    let m = DMatrix::<f64>::identity(n, n);

    let lup = LU::new(m.clone());
    let b1 = DVector::identity(n);
    let sol1 = lup.solve(&b1).unwrap();

    println!("{}", sol1);
}
