enum List {
    Node(i32, Box<List>),
    Nil,
}

pub fn run() {
    // let a1: [u8; 9000000] = [1; 9000000];

    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];

    println!("Static memory address of v1 is: {:p}", &v1);
    println!("Static memory address of v2 is: {:p}", &v2);

    v1.insert(1, 3);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);

    let t1: (i64, String) = (10, String::from("hello"));
    println!("Static address of tapple data is: {:p}", &t1);
    println!("Static memory address of t1.1: {:?}", t1.1.as_ptr());
    println!("Len of t1.1: {:?}", t1.1.len());
    println!("Capacity of t1.1: {:?}", t1.1.capacity());

    let mut b1 = Box::new(t1);
    (*b1).1 += "world";
    println!("{} {}", b1.0, b1.1);
    println!("Static address of box pointer is: {:p}", &b1);
    println!("Heap address of tapple data is: {:p}", b1);
}
