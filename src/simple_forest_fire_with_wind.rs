use crate::simple_forest_fire::Forrest;

#[test]
fn wind_example() {
    let forrest_example = Forrest::new(100, 0.5);

    println!("{}", forrest_example.cells());
    println!("{}", forrest_example.no_clusters())
}
