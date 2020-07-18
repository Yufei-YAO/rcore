// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// Steps:
// 1. If the length of the provided string is 0, then return an error
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. If the name is empty, then return an error
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
//    with something like `"4".parse::<usize>()`.
// If while parsing the age, something goes wrong, then return an error
// Otherwise, then return a Result of a Person object
impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() == 0{
            Err("err".to_string())
        }else{
            let k =String::from(s);
            let st =k.as_bytes();
            let mut i:usize =0;
            while i<st.len() &&st[i] as char !=','
            {
                i+=1;
            }
            if i==st.len(){
                Err("err".to_string())
            }
            else {
                if(s == "Mark,20"){
                    println!("1");
                }
            let mut name  =  String::from(s);
            let ag_string = name.split_off(i);

            
            if(name.is_empty()){

                Err("err".to_string())
            }else{
                let mut ag:usize =0 ;
                let mut flag:bool =true;
                let mut st = ag_string.bytes();
                let mut h =true;
                let mut d =false;
                for i in st{
                    if h==true{
                        h =false;
                        continue;
                    }
                    if i<48 || i>73{
                        flag=false;
                        break;
                    }
                    ag=ag*10usize+(i as usize-48);
                    d =true;
                    println!("{}",ag);
                }
                

                if flag==true && d==true{
                    Ok(Person{name:name,age:ag})
                }else{
                    Err("err".to_string())
                }
                 
            }
        }
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John,".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_age() {
        "John,twenty".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_comma_and_age() {
        "John".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name() {
        ",1".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_age() {
        ",".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_invalid_age() {
        ",one".parse::<Person>().unwrap();
    }

}
