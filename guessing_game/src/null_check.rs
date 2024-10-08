pub fn check_number() {
    let number = Some(10);

    match number {
        Some(n) => println!("The number is: {}", n),
        None => println!("No number provided"),
    }

    if let Some(n) = number {
        println!("The number is: {}", n);
    } else {
        println!("No number provided");
    }
}
