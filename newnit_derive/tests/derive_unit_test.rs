use newnit::Unit;
use newnit_derive::Unit;

#[test]
fn derived_struct() {
    #[derive(Unit)]
    #[unit(factor = 1.0)]
    struct Newnit(f64);
}
