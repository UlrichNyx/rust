/*
Chapter 6: Enums and Pattern Matching
Challenge: Create an enum to represent different kinds of transport (e.g., Car, Bike, Bus, Walking) and write a
function that returns the time it would take to travel a certain distance using that transport.
Use pattern matching to implement the logic.
 */


enum Transportation {
    Car,
    Bike,
    Bus,
    Walking


}

impl Transportation {
    fn transportation_time(&self) -> u32{
        match self {
            Transportation::Bike => 25,
            Transportation::Car => 20,
            Transportation::Bus => 22,
            Transportation::Walking => 40
        }
    }
}




fn main() {
    println!("{}", Transportation::Car.transportation_time());
    println!("{}", Transportation::Bike.transportation_time());
    println!("{}", Transportation::Bus.transportation_time());
    println!("{}", Transportation::Walking.transportation_time());
}


