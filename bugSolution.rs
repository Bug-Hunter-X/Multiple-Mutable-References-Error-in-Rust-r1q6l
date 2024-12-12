fn main() {
    let mut x = 5;
    { // Create a new scope
        let y = &mut x;
        *y += 1;
    }
    { // Create a second scope
        let z = &mut x; 
        *z += 1;
    }
    println!("x = {}", x);
}