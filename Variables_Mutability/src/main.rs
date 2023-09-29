fn main() {
    // Variables
    let x = 5;
    println!("Immutable x:{}", x);
    // Immutable
    // x = 6; // CTE: Cannot assign a new value to an immutable variable

    // Mutable
    let mut x1 = 5;
    println!("Mutable x1:{}", x1);
    x1 = 6;
    println!("Mutable x1:{}", x1);

    // Shadowing
    let x = 6;
    println!("Immutable x:{}", x);

    // Constants
    const A_CONST: i32 = 25;
    println!("Constant A_CONST:{}", A_CONST);
}
