fn main() {
    let tuple: (&str, i32, bool, f64) = ("first element", 2, true, 4.3);

    println!("Elemento 1: {}, Elemento 2: {}", tuple.0, tuple.1)
}