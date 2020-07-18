

fn main()
{
    let mut numbers:[i32;4] = [0;4];
    let mut name:[u8;4] =  ['a' as u8;4];

    // first, print them out raw
    println!("numbers: {} {} {} {}",
            numbers[0], numbers[1], numbers[2], numbers[3]);

    println!("name each: {} {} {} {}",
            name[0], name[1], name[2], name[3]);

    println!("name: {}", String::from_utf8_lossy(&name));

    // setup the numbers
    numbers[0] = 1;
    numbers[1] = 2;
    numbers[2] = 3;
    numbers[3] = 4;

    // setup the name
    name[0] = 'Z' as u8;
    name[1] = 'e' as u8;
    name[2] = 'd' as u8;
    name[3] = 'A' as u8;

    // first, print them out raw
    println!("numbers: {} {} {} {}",
            numbers[0], numbers[1], numbers[2], numbers[3]);

    println!("name each: {} {} {} {}",
            name[0], name[1], name[2], name[3]);

    println!("name: {}", String::from_utf8_lossy(&name));

    // another way to use name
    let another = "Zed ";

    println!("another: {}", another);
    let h =another.as_bytes();
    println!("another each: {} {} {} {}",
            h[0], h[1], h[2], h[3]);

    return ;
}