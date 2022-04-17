pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;

    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("Static memory address of i1 is: {:p}", &i1);
    println!("Static memory address of i2 is: {:p}", &i2);

    let sl1 = "literal";
    let sl2 = sl1;
    println!("Static memory address of sl1 is: {:p}", &sl1);
    println!("Static memory address of sl2 is: {:p}", &sl2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("Static memory address of s3 is: {:p}", &s3);
    println!("Static memory address of s4 is: {:p}", &s4);
    println!("Heap address of s3 data is: {:p}", s3.as_ptr());
    println!("Heap address of s4 data is: {:p}", s4.as_ptr());
    println!("{} {}", s3, s4);

    let s5 = String::from("hello");
    take_ownership(s5);
    // println!("{}", s5);

    let s6 = String::from("hello");
    let s7 = take_giveback_ownership(s6);

    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The length of '{}' is {}.", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // let r2 = &mut s10;
    // println!("{} {}", r1, r2);
}

fn take_ownership(s: String) {
    println!("{}", s)
}

fn take_giveback_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}
