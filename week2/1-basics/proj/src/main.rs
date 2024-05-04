fn main() {
    let message = "Name: Antonio Masotti";
    let age = 30;
    let weight_lbs = 190.0;

    let kilos = weight_lbs / 2.2046;

    println!("{}", message);
    println!("Age: {}", age);
    println!("Weight: {:.2} kg", kilos);
}
