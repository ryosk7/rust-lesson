// pub mod sub_a;
// pub mod sub_b;

const _MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module!!!");
    // sub_a::func_a();
    // sub_b::func_b();
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);
    println!("Memmory address of const is: {:p}", &_MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Memmory address of i2 is: {:p}", &i2);
    println!("Memmory address of i3 is: {:p}", &i3);

    let y = 6;
    println!("Stack address of y is: {:p}", &y);
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of x is {}", y);

    let t1 = (500, 6.4, "dummy");
    let (_x, _y, _z) = t1;
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));

    let ((ref mut _x1_ptr, ref mut _y1_ptr), _) = t2;
    *_x1_ptr = 5;
    *_y1_ptr = -5;
    println!("{:?}", t2);

    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a2[3]);

    let s1 = "helloこんにちは挨拶";
    let s2 = "hello";
    println!("Memmory address of s1 is: {:p}", &s1);
    println!("Memmory address of s2 is: {:p}", &s2);
    println!("Static memory address of s1 is: {:?}", &s1.as_ptr());
    println!("Static memory address of s2 is: {:?}", &s2.as_ptr());
    println!("Len of s1 is: {:?}", &s1.len());
    println!("Len of s2 is: {:?}", &s2.len());

    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");

    println!("Static memory address of s1 is: {:?}", &s1.as_ptr());
    println!("Static memory address of s2 is: {:?}", &s2.as_ptr());

    s1.push_str("hoge");
    s2.push_str("hoge2");
    println!("{} {}", s1, s2);
}
