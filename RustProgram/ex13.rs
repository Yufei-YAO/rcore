fn main()
{
    let mut  i = 0;

    // go through each string in argv
    // why am I skipping argv[0]?
    
    let args = std::env::args();
    for arg in args {
        println!("arg {}: {}\n", i, arg);
        i+=1;
    }

    // let's make our own array of strings
    let states:Vec<&str> = vec![
        "California", "Oregon",
        "Washington", "Texas"
    ];

    let  num_states = 5;

    
    for i in 0..num_states as usize{
        println!("state {}: {}", i, states[i]);
    }

}