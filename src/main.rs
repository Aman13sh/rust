/*
Declare a `season` variable set to a string with
your favorite season. Provide an explicit type annotation.
The type of a string is a `&str`. We'll discuss what
the & symbol means later in the course.
 
Declare a `points_scored` variable set to 28.
Provide an explicit type annotation. The type of
an integer is `i32`.
 
It's time to update the team's score. Declare the
`points_scored` variable to be mutable. Set its
new value to 35.
 
Declare a `TOUCHDOWN_POINTS` constant at the file
level set to the value 6.
 
Declare a `event_time` variable set to a string of
"06:00".
 
Use variable shadowing to redeclare `event_time` set
to a integer of 6.
 
Use interpolation to print out all of the
declared variables and constants in a println! call.
Practice using direct interpolation {value}, sequential
arguments ( {} ), and numeric arguments ( {0} ).
 
Declare a `favorite_beverage` variable set to a string
of your favorite drink. Use an underscore to silence
the compiler warning about the variable being unused.
 
Remove the underscore. Provide a compiler directive
to silence the compiler warning about the variable
being unused.
*/

// use std::ops::Index;

// const TOUCHDOWN_POINTS: i32 = 6;

fn main(){
    
    // variable declaration in rust variable declaration is similar to
    // any statically type language all variables are immutable need mut key word to make a variable mutable.
    // let season : &str = "Summer";

    // let mut points_scored = 28;
    // points_scored = 35;

    // #[allow(unused_variables)]
    // let event_time = "06:00";

    // let event_time = 6;

    // println!("{} {} {} {}", season , points_scored , event_time,TOUCHDOWN_POINTS);





    // let str1 = String::from("Aman");
    
    // println!("{} {}",str1 , &str1[0..2])

    //immutable borrowing
    // let car:String = String::from("Red");
    // let ref1: &String= &car;
    // let ref2: &String = &car;
    // println!("{ref1} and {ref2} and {}",&car);



    //mutable borrow

    // let mut car:String = String::from("Red");
    // let ref1: &mut String = &mut car ;
    // ref1.push_str("and Silver");
    // let ref2:&String = &car;


    let food: &str = "pizza";
    let pizz_slice: &str = &food[0..3];

    println!("{}",pizz_slice.len());




}