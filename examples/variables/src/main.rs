

fn to_kilos() {
    let message = "Name: Alfredo, Weight: ";
    let weight = 190.0;

    let kilos = weight / 2.2;
    println!("{}{}", message, weight);
    println!("{}{}", message, kilos);
    
}

fn main() {
    let mut message = String::from("Name: Alfredo, Height: ");
    message.clear();
    let mut height = 190;
    height = 189;
    println!("{}{}", message, height);

}