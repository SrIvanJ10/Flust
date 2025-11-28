fn main() {
    let sensor = 25.0; // Mock input
    let threshold = if sensor > 30.0 { 1.0 } else { 0.0 };
    println!("Alarm: {}", threshold);
}
