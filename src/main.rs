fn is_even(num: i32) -> bool {
    if num % 2 == 0{
        return true;
    }
    return false;
}

fn fib( num: u32) -> u32 {
    if num <= 1 {
        return num
    }
    return fib( num - 1 ) + fib( num - 2 ); 
}

fn fib2 ( num: u32 ) -> u32 {
    let mut  first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }
     
    if num == 1 {
        return  second;
    }
    
    for _ in 0..(num-1) {
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second;
}

fn get_str_length(str: String) ->usize{
    str.chars().count()
}

fn main() {
    println!("{}", is_even(7));
    println!("{}",fib(5) );
    println!("{}", fib2(5));
    let name = String::from("saurabh");
    let len = get_str_length(name);
    println!("{}", len);
}
