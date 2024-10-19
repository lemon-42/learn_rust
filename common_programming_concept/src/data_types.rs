pub fn data_types() {
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
    let _forty_two = 42u8;
    let _decimal = 98_222;
    let _hexadecimal = 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let _octal_two = b'A'; // u8 only

    // Uncommenting the following lines will result in a compilation error
    // Due to buffer and under overflow

    /* let buffer_overflow: u8 = 255 + 1;
       let buffer_underflow:u8 = 0 - 1;  */

    /*          Float types              */

    let _x = 2.0;
    let _y: f32 = 3.0;

    /*          Numeric Operation        */
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
    let _remainder = 43 % 5;

    /*          Boolean types            */
    let _t = true;
    let _f: bool = false;

    /*          Character types          */
    let _c = 'z';
    let _penguin = '🐧';
}
