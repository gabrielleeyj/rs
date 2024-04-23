fn main() {
    // boolean
    let boo: bool = true;

    // uint
    let i1: u8 = 1;
    let i2: u16 = 1;
    let i3: u32 = 1;
    let i4: u64 = 1;
    let i5: u128 = 1;

    // sint
    let i7: i8 = 1;
    let i8: i16 = 1;
    let i9: i32 = 1;
    let i10: i64 = 1;
    let i11: i128 = 1;

    // floats
    let f1: f32 = 1.0;
    let f2: f64 = 2.0;

    // platform specific int
    let p1: usize = 1;
    let p2: usize = 2;

    // chars
    let chars: char = 'c';

    // str
    let strs: &str = "yo";

    // String
    let string: String = String::from("yo");

    // arrays where you can specify [type; size]
    let arr: [i32; 5] = [1,2,3,4,5];

    let i1: i32 = arr[4];

    // let rust infer type
    let arrs = [1,2,3,4,5];
    let i2 = arr[1];

    // tuples
    let t1: (i32, i32, i32) = (1,2,3);
    let t1 = (5,5.0,"5");

    let s1 = t1.2;
    let (i1,f1,s1) = t1;

    let unit = ();

    // type aliasing
    type age = u8;
    
    let a1: age = 57;
}
