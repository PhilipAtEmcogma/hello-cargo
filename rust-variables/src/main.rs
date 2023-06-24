fn main() {
    //x and y is immutable (i.e. can't reassign)
    //if want x and y to bne mutable, use let mut x = 2 instead
    //i means signed integer
    let x = 2;
    let y = 5;
    println!("Hello, world! {} + {} = {}",x,y,x+y);

    //u mean unsigned, means can't be negative
    let a: u8 = 255;
    let b: u16 = 65535;
    let c: u32 = 4294967295;
    let d: u64 = 18446744073709551615;

    println!("{}, {}, {}, {}", a, b, c, d);

    //for unsigned integers
    let a_min: u8 = std::u8::MIN;
    let b_min: u16 = std::u16::MIN;
    let c_min: u32 = std::u32::MIN;
    let d_min: u64 = std::u64::MIN;
    let e_min: usize = std::usize::MIN;

    let a_max: u8 = std::u8::MAX;
    let b_max: u16 = std::u16::MAX;
    let c_max: u32 = std::u32::MAX;
    let d_max: u64 = std::u64::MAX;
    let e_max: usize = std::usize::MAX; //usize will return whatever size the arciture the machine have, e.g. 32 or 64bit

    println!("{}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("{}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);

    //for signed integers
    let a_min: i8 = std::i8::MIN;
    let b_min: i16 = std::i16::MIN;
    let c_min: i32 = std::i32::MIN;
    let d_min: i64 = std::i64::MIN;
    let e_min: isize = std::isize::MIN;

    let a_max: i8 = std::i8::MAX;
    let b_max: i16 = std::i16::MAX;
    let c_max: i32 = std::i32::MAX;
    let d_max: i64 = std::i64::MAX;
    //if unsure how big the variable need, just use isize
    let e_max: isize = std::isize::MAX; //usize will return whatever size the arciture the machine have, e.g. 32 or 64bit

    println!("{}, {}, {}, {}, {}", a_min, b_min, c_min, d_min, e_min);
    println!("{}, {}, {}, {}, {}", a_max, b_max, c_max, d_max, e_max);

    //floating point
    let a_min: f32 = std::f32::MIN;
    let b_min: f64 = std::f64::MIN;

    let a_max: f32 = std::f32::MAX;
    let b_max: f64 = std::f64::MAX;

    println!("{}, {}", a_min, b_min);
    println!("{}, {}", a_max, b_max);

    //to initialise a float number, must have a number after the decimal e.g. 1.0
    let a: f64 = 1.0;
    let b: f64 = 0.1;
    let c: f64 = 0.2;
    println!("{}, {}", a, b+c);

    //char variables
    let ch1: char = 'X';
    //char unicode
    let ch2 = '\u{2603}';

    println!("{}, {}", ch1, ch2);t

    //boolean variables, note they're imutable by default, 
    //so a boolean variable need it to be mutable, must add mut
    let on: bool = true;
    let off: bool = false;

    println!("{},{},{}", on, off, !on);

    //constant, note: constant must have a decalre type
    //also a can't assign non constant variable to a constant
    const PI: f64 = 3.1415;

    println!("{}", PI);
}
