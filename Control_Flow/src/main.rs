fn main() {
    // IF
    let number: u8 = 1;
    let condition = true;
    if number % 2 == 0 {
        println!("even");
    } else if number % 7 == 0 {
        println!("Lucky")
    } else {
        println!("odd");
    }

    let vd = if condition { 5 } else { 6 };
    println!("vd = {}", vd);

    /* LOOPS */
    // Loop
    let mut counter = 5;
    loop {
        counter -= 1;
        println!("Forever!");
        if counter < 0 {
            break;
        }
    }
    // While
    while counter > 0 {
        counter -= 1;
        println!("{}", counter);
    }
    // For
    for i in 1..4 {
        println!("{}", i);
    }
}
