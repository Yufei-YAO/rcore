struct State{
    the_size:i32,
    the_age:i32,
}


impl State{
fn get_age(&mut self)->i32
{
    return self.the_age;
}

fn set_age(&mut self, age:i32)
{
   self.the_age = age; 
}
}

fn update_ratio(new_ratio:f64)->f64
{
    let mut ratio:f64 = 1.0;

    let old_ratio:f64 = ratio;
    ratio = new_ratio;

    return old_ratio;
}

fn print_size(THE_SIZE:i32)
{
    println!("I think size is: {}", THE_SIZE);
}

fn scope_demo(mut count:i32)
{
    println!("count is: {}", count);

    if count > 10 {
       let numbers = 100;	// BAD! BUGS!

        println!("count in this scope is {}", numbers);
    }

    println!("count is at exit: {}", count);

    count = 3000;

    println!("count after assign: {}", count);
}

fn main()
{
    // test out THE_AGE accessors
    let mut MY_NAME:&str = "Zed A. Shaw";
    let  mut THE_SIZE:i32=5;
    let mut s =State{the_size:THE_SIZE,the_age:10};
    println!("My name: {}, age: {}", MY_NAME, s.get_age());

    s.set_age(100);

    println!("My age is now: {}", s.get_age());

    // test out THE_SIZE extern
    println!("THE_SIZE is: {}", THE_SIZE);
    print_size(THE_SIZE);

    THE_SIZE = 9;

    println!("THE SIZE is now: {}", THE_SIZE);
    print_size(THE_SIZE);

    // test the ratio function static
    println!("Ratio at first: {}", update_ratio(2.0));
    println!("Ratio again: {}", update_ratio(10.0));
    println!("Ratio once more: {}", update_ratio(300.0));

    // test the scope demo
    let mut count = 4;
    scope_demo(count);
    scope_demo(count * 20);

    println!("count after calling scope_demo: {}", count);

    return ;
}