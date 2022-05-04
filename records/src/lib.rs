use std::io;



struct VisitorsName{
    rooms:i32,

}


fn main() {
    
    let mut first_name = String::new(); // make a mutable string variable
    println!("Enter the first name please");
    io::stdin().read_line(&mut first_name); //to get input from the user
    let first_name: String= first_name.trim().parse().unwrap(); // performing shadowing to convert the type 
    

    let mut second_name = String::new(); // make a mutable string variable
    println!("Enter the second name please");
    io::stdin().read_line(&mut second_name); //to get input from the user
    let second_name: String = second_name.trim().parse().unwrap();// performing shadowing to convert the type 
    
    let mut id= String::new(); // make a mutable string variable
    println!("Enter id no please");
    io::stdin().read_line(&mut id); //to get input from the user
    let id: i32 = id.trim().parse().unwrap();// performing shadowing to convert the type 
    


    let mut age = String::new(); // make a mutable string variable
    println!("Enter age please");
    io::stdin().read_line(&mut age); //to get input from the user
    let age: i32= age.trim().parse().unwrap();// performing shadowing to convert the type 
    
    

    let mut county_of_origin = String::new(); // make a mutable string variable
    println!("Enter the county you are from please");
    io::stdin().read_line(&mut county_of_origin); //to get input from the user
    let county_of_origin: String = county_of_origin.trim().parse().unwrap();// performing shadowing to convert the type 
     //fn main() {
        //let mut rooms =100;
        //for  mut rooms in 0..100 {
            //if rooms ==0{
                //println!("all the rooms are fully booked");
            //}else{
               // println!("rooms remaining are{}",)
    
                
            //}
            //rooms -= 1;
       // }
     //}
    

    panic!("the following person {:?}of id {:?} ,age{:?} from county {:?}to proceed to room no:  ",first_name + &second_name ,id, age ,county_of_origin); 
    //let rooms =100;
    //for room in rooms {
        //sorted_rooms.sort_by(|0..100|)
    //}
    

    
}