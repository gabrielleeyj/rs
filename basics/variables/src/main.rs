fn main() {
    // creation
    let a: i16 = 5;

    // mutability
    let mut b: i32 = 5;
    b = 10;
    // shaodowing
    let c: i32 = 10;
    println!("c is: {c}");

    let c: i32 = 20;
    println!("c is: {c}");
    // scope
    let d: i32 = 30;

    {
        let d: i32 = 40;
        println!("inner d is: {d}"); // 40
    }

    println!("d is: {d}"); // this will print 30
    
    // e cannot be accessed in the outerscope if we do this
    {
        let e: i32 = 40;
        println!("inner e is: {e}");
    }
    // this will cause an error
    println!("outer e is: {e}");
}
