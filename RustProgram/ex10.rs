  


fn  main()
{
    let mut args = std::env::args();
    args.next();
    if let Some(i) = args.next()  {
        let h = i.to_string();
        let mut bytes = h.bytes();
        let mut  i= 0;
        while let Some(f) = bytes.next(){
            match f as char {
                'a' | 'A' => println!("{}:'A' ",i),
                'e' | 'E' => println!("{}:'E' ",i),
                'i' | 'I' => println!("{}:'I' ",i),
                'o' | 'O' => println!("{}:'O' ",i),
                'u' | 'U' => println!("{}:'U' ",i),
                'y' | 'Y' => println!("{}:'Y' ",i),
                _ =>  println!("{} : {} is not a vowel", i, f as char),
            }
            i+=1;
        }
    }else{
        println!("ERROR: You need one argument.");
        // this is how you abort a program
        return ();
    }


}