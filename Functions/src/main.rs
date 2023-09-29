fn main() {
    let mut x:u8 = 1;
    /* FUNCTIONS */
    println!("Before of function {}",x);
    x = newfunc(x);
    println!("After of function {}",x);

    // Statements => Ends with ';' and don't return a value.

    // Expression => Returns a value and no ';'.

}

fn newfunc(x : u8) -> u8{
    println!("== Inside of function {}",x);
    x+1 // OR `return x+1;`
}
