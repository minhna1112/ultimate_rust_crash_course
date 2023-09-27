
const STARTING_MISSLES : i32 = 8;
const READY_AMOUNT : i32 = 2;

fn main() {
    let name = "Stalin";
    let  (mut missiles, ready) : (i32, i32) = (STARTING_MISSLES, READY_AMOUNT);
    
    println!("Firing {} of {} my missions", ready, missiles);
    println!("{} missiles left", missiles - ready);
    READY_AMOUNT = 1;
// }
