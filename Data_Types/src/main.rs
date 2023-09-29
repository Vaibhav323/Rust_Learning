fn main() {
    // Explicit VS Implict
    let mut e: u8 = 0; // Explicit Data Type declaration
    let mut i = 0; // Implicit Data Type declaration

    /* Scalar Types */
    // Integers
    /// Unsigned VS Signed
    let mut int: u8 = 255;

    // Floats
    let mut float = 2.0; // f64-> default

    // Boolean
    let mut b: bool = true; // Must be defined explicitly

    // Char
    let mut c = '🦀'; // Size of char is 4 bytes
    println!("{}", c);

    /* Compound Types */
    // Tuples => Consist of different data types
    let mut tup = (1, 2, 'c');
    println!("{:?}", tup);
    // tup.1 = '🦀'; // It is not allowed as the '1's placed as an integer.
    tup.2 = '🦀'; // It is valid as it is replacing char's location
    println!("{:?}", tup);

    let (x, y, z) = tup;
    println!("{}", x);
    // We can have tuple inside tuple
    let mut tup1 = (tup, 1);
    println!("{:?}", tup1);
    println!("{}", (tup1.0).2); // Access tuple inside a tuple

    // Array => Consist of Same data types
    let mut arr = [1, 2, 3]; //Implicit
    let mut arr1: [u8; 3] = [1, 2, 3]; // Explicit
    println!("{:?}", arr1);
    arr1[0] = 20;
    println!("{:?}", arr1);
    /* Curiosities */
    // Two's Complement
    /***************************************
    If we take 1 in bit i.e. 00000001
    and we reverse them 11111110
    and add 1 in it i.e. 11111111
    Then it is 2's complement of 1.
     ***************************************/
    // Overflow
    let mut unsigned: u8 = 255; // u8 -> 0-255
    unsigned = unsigned + 1;
    println!("{}", unsigned);   // It wraps the 255->0 in release version
}
