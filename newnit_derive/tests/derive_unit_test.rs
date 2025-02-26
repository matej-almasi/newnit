use newnit::Unit;
use newnit_derive::Unit;

#[test]
fn derived_struct() {
    #[derive(Unit)]
    #[unit(factor = 1.0)]
    struct Newnit(f64);

    let newnit = Newnit(42.0);
    assert_eq!(newnit.to_base(), 42.0);
}
