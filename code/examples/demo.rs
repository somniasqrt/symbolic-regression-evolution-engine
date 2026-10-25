use symreg::dataset::Dataset;

fn main() {
    let ds = Dataset::new(
        vec!["x".to_string(), "y".to_string()],
        vec![vec![1.2, 1.15], vec![2.0, 2.0]],
        "z".to_string(),
        vec![2.5, 4.0],
    )
    .unwrap();
    println!("{ds}")
}
