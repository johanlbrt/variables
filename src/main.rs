fn main() {
    x_value(5);
}

fn x_value(x: i32) {
    let text = String::from("The value of x is:");
    println!("{text} {x}");
}