// fn is_even(num: i32) -> bool {
//     if num % 2 == 0{
//         return true;
//     }
//     return false;
// }

// fn fib( num: u32) -> u32 {
//     if num <= 1 {
//         return num
//     }
//     return fib( num - 1 ) + fib( num - 2 ); 
// }

// fn fib2 ( num: u32 ) -> u32 {
//     let mut  first = 0;
//     let mut second = 1;

//     if num == 0 {
//         return first;
//     }
     
//     if num == 1 {
//         return  second;
//     }
    
//     for _ in 0..(num-1) {
//         let temp = second;
//         second = second + first;
//         first = temp;
//     }

//     return second;
// }

// fn get_str_length(str: String) ->usize{
//     str.chars().count()
// }

// fn main() {
//     println!("{}", is_even(7));
//     println!("{}",fib(5) );
//     println!("{}", fib2(5));
//     let name = String::from("saurabh");
//     let len = get_str_length(name);
//     println!("{}", len);
// }

// struct User {
//     name: String,
//     age: i32
// }

// implementing structs

// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     // let user = User {
//     //     name: String::from("saurabh"),
//     //     age: 33,
//     // };
//     // println!("{} {}", user.name, user.age);

//     let rect = Rect {
//         width: 30,
//         height: 40
//     };
//     println!("the area of reactangle is {}", rect.area())
// }


//Enums

enum Shape {
    Reactangle(f64, f64),
    Circle(f64),
}

fn main() {
    let rect = Shape::Reactangle(1.0, 3.0);
    calculate_area(rect);
    let circle = Shape::Circle(2.0);
    println!("{}",calculate_area(circle))
}

fn calculate_area(shape: Shape) -> f64 {
    match shape{
        Shape:: Reactangle(a,b ) => a*b,
        Shape::Circle(r) => 3.14 * r* r
    }
}
