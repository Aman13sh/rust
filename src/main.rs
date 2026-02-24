
// use std::ops::Index;

// const TOUCHDOWN_POINTS: i32 = 6;

// struct Coffee{
//     price:f64,
//     name:String,
//     is_hot:bool
// }

#[derive(Debug)]
struct TaylorSwifitSong{
    title: String,
    realease_year: u32,
    duration_secs:u32
}

impl TaylorSwifitSong{
    fn display_song_info(self){
        println!("{}" , self.title);
        println!("{}",self.realease_year);
        println!("{}",self.duration_secs);
    }

}



fn main(){



    let song :TaylorSwifitSong = TaylorSwifitSong { title: String::from("Aman"), realease_year: 10, duration_secs: 5 };
    println!("{:?}",song);

    song.display_song_info();

   
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

    //slices

    // let food: &str = "pizza";
    // let pizz_slice: &str = &food[0..3];

    // println!("{}",pizz_slice.len());

    // let name:String =  String::from("Latte");
    // let cofee:Coffee = make_coffee(name , 5.99 , true);

    // let caramel_macchiato:Coffee = Coffee{
    //     name :String::from("Aman"),
    //     ..cofee        
    // };

    // println!("{} {} {}",cofee.name,cofee.price,cofee.is_hot)



}


// fn make_coffee(name:String , price:f64 , is_hot:bool) -> Coffee{
//     Coffee{
//         name:name,
//         price:price,
//         is_hot:is_hot
//     }
// }