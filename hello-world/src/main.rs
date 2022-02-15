fn print_type_of<T>(_: &T) {
    println!("The best type for x is: {}", std::any::type_name::<T>())
}

fn main() {
    println!("Hello, world!");

    let mut x = 4;
    println!("The value of x is 4, right? This is {}", x == 4);

    println!("Adding 1 to x...");
    x += 1;
    println!("The new value of x is 5, right? This is {}", x == 5);
    print_type_of(&x);

    let tuple_1 = (1, "hello", 4.5, true);
    println!("The value of tuple_1 is: {:?}", tuple_1);
    println!("The value of the second element of tuple_1 is: {:?}", tuple_1.1);

    let array_0 = [1, 2, 3, 4, 5];
    println!("The value of array_0 is: {:?}", array_0);
    println!("The value of the second element of array_0 is: {:?}", array_0[1]);
    
    if array_0.len() == 5 {
        println!("The array has 5 elements!");
    } else {
        println!("The array has {} elements!", array_0.len());
    }

    loop {
        println!("This loop will run forever! unless you break it!");
        println!("Let's break it");
        if 1 > 0 {
            break;
        }}

    while x < 10 {
        println!("The value of x is: {}", x);
        x += 1;
    }

    for index in 0..10 {
        println!("The value of index is: {}", index);
    }

    for index in 0..=10 {
        println!("The new value of index is: {}", index);
    }

    for x in array_0 {
        println!("The value of the element in array_0 is is: {}", x);
    }

    match x {
        1 => println!("The value of x is 1"),
        2 => println!("The value of x is 2"),
        3 => println!("The value of x is 3"),
        _ => println!("The value of x is not 1, 2, or 3, it is: {}", x),
    }
}
