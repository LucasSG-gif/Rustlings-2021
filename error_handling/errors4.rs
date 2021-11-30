// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)

// I AM DONE

use std::num::ParseIntError;

fn main() -> Result<(),ParseIntError>{
    let mut tokens = 100;
    let pretend_user_input = "8";
    let cost = total_cost(pretend_user_input);
    match cost {
        Ok(x) if x > tokens => 
            println!("You can't afford that many!"),
        Ok(x) => { 
            tokens -= x;
            println!("You now have {} tokens.", tokens);
        },
        Err(e) => (), 
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)    
}
