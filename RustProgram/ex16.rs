
struct Person {
    name:String,
    age:i32,
    height:i32,
    weight:i32,
}

fn Person_create(na:&str,ag:i32,he:i32,we:i32)->Box<Person>{
    assert!(!(na.len()==0));
    Box::new(Person{name:na.to_string(),age:ag,height:he,weight:we})
}




fn Person_print(who:&Box<Person>)->()
{
    println!("Name: {}\n", who.name);
    println!("\tAge: {}\n", who.age);
    println!("\tHeight: {}\n", who.height);
    println!("\tWeight: {}\n", who.weight);
}

fn main()
{
    // make two people structures
    let mut joe = Person_create("Joe Alex", 32, 64, 140);

    let mut frank = Person_create("Frank Blank", 20, 72, 180);

    // print them out and where they are in memory
    println!("Joe is at memory location {:p}:\n", joe);
    Person_print(&joe);

    println!("Frank is at memory location {:p}:\n", frank);
    Person_print(&frank);

    // make everyone age 20 years and print them again
    joe.age += 20;
    joe.height -= 2;
    joe.weight += 40;
    Person_print(&joe);

    frank.age += 20;
    frank.weight += 20;
    //free(frank);
    //Person_print(frank);

}