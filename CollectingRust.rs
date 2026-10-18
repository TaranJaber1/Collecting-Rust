struct Card {
    name: String,
    set: String,
    quantity: u32,                        // unsigned 32 bit integer (no negative number)
    value: f64,                           // floating point 64 bit, capable of storing large values and decimal places
}

fn main (){
    let charizard = Card {
        name: String::from("Mega Charizard Ex"),
        set: String::from("Phantasmal Flames"),
        quantity: 1,
        value: 600.00,
    };

    println!("Card Name: {}", charizard.name);
    println!("Set: {}", charizard.set);
    println!("Quantity: {}", charizard.quantity);
    println!("Value: ${:.2}", charizard.value);
}