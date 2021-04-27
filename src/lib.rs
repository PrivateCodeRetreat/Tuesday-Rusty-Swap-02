#![cfg_attr(feature = "strict", deny(warnings))]

pub fn set_alive() {}
pub fn next_state(is_alive: bool, number_of_neighbors: i32) -> bool {
    if number_of_neighbors == 2 && is_alive == false {
        false
    } else if number_of_neighbors == 2 || number_of_neighbors == 3 {
        true
    } else {
        false
    }
}
