#[allow(unused_variables)]

pub fn scalar() {
    /*          Integers Types
     *
     *  |  Size     |  Signed  |  Unsigned  |
     *  |  8 bits   |   i8     |     u8     |
     *  |  16 bits  |   i16    |     u16    |
     *  |  32 bits  |   i32    |     u32    |
     *  |  64 bits  |   i64    |     u64    |
     *  |  128 bits |   i128   |     u128   |
     *  |  arch     |   isize  |     usize  |
     *  
     *  Signed   : Can take negatives values
     *  Unsigned : Can take only positive values
     */

    // We can write this to specify the type
    let forty_two = 42u8;
    let decimal = 98_222;
    let hexadecimal = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let octal_two = b'A'; // u8 only

    // Uncommenting the following lines will result in a compilation error
    // Due to buffer and under overflow

    /* let buffer_overflow: u8 = 255 + 1;
       let buffer_underflow:u8 = 0 - 1;  */

    /*          Float types              */

    let x = 2.0;
    let y: f32 = 3.0;

    /*         Numeric Operation        */
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;

    /*          Boolean types            */
    let t = true;
    let f: bool = false;

    /*         Character types          */
    let c = 'z';
    let penguin = '🐧';
}
