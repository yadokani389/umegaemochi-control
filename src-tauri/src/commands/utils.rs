pub fn stringify(e: impl ToString) -> String {
    println!("Error: {}", e.to_string());

    e.to_string()
}
