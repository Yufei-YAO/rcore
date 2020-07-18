

/** Our old friend die from ex17. */
fn die(message:&str)
{


    println!("ERROR: {}", message);
    panic!();
}

// a typedef creates a fake type, in this
// case for a function pointer


/**
 * A classic bubble sort function that uses the 
 * compare_cb to do the sorting. 
 */


fn bubble_sort(numbers:& [i32],count:i32, cmp:fn(i32,i32)->bool)->[i32;500]
{
    let mut temp:i32 = 0;
    let mut i:usize = 0;
    let mut j:usize= 0;
    let mut target:[i32;500]=[0;500];

    for i in 0..numbers.len(){
        target[i] = numbers[i];
    }
    for i  in 0..count as usize{
        for j in 0..(count-1) as usize {
            if cmp(target[j],target[j+1]) == true {
                temp = target[j + 1];
                target[j + 1] = target[j];
                target[j] = temp;
            }
        }
    }
    return target;
}

fn sorted_order(a:i32, b:i32)->bool
{
    if (a - b)>0{
        true
    }else{
        false
    }
}


fn reverse_order(a:i32, b:i32)->bool
{
    if (a - b)>0{
        false
    }else{
        true
    }
}

fn strange_order(a:i32, b:i32)->bool
{
    if (a==0||b==0){
        false
    }else{
        if a%b==0{
            false
        }else{
            true
        }
    }
}


/** 
 * Used to test that we are sorting things correctly
 * by doing the sort and printing it out.
 */
fn test_sorting(numbers:&[i32], count:i32, cmp:fn(i32,i32)->bool)
{
    let i:i32 = 0;
    let sorted = bubble_sort(numbers, count, cmp);

    // if (!sorted)
    //     die("Failed to sort as requested.");

    for i in 0..count {
        print!("{} ", sorted[i as usize]);
    }

    println!(" ");
}


fn main()
{
    //if (argc < 2) die("USAGE: ex18 4 3 1 5 6");
    let numbers:[i32;5] = [4,3,1,5,6];
    let count = 5;


    test_sorting(&numbers, count, sorted_order);
    test_sorting(&numbers, count, reverse_order);
    test_sorting(&numbers, count, strange_order);
}