fn main() {
    let mut text = String::from("The x is:");

    let x = 5;

    println!("{text} {x}");

    let x = x + 5;

    text = String::from("Then x is:");

    println!("{text} {x}");
}
