#![cfg_attr(feature = "strict", deny(warnings))]

pub fn set_alive() {}

pub fn next_state(is_alive: bool, number_of_live_neighbors: i32) -> bool {
    if number_of_live_neighbors == 2 && is_alive == false {
        false
    } else if number_of_live_neighbors == 2 || number_of_live_neighbors == 3 {
        true
    } else {
        false
    }
}

pub fn calculate_live_neighbors(neighbors: [bool; 8]) -> i32 {
    let mut count = 0;

    for neighbor in neighbors.iter() {
       if *neighbor{
           count= count+1;
       }
    }
    return count;
}
pub struct Board{
   stuff: Vec<[i32;2]>
}
impl Board{
    pub fn new() -> Self{
        return Board{
            stuff : Vec::new()
        };
    }

    pub fn is_alive(&self, x:i32, y:i32) -> bool {
        return self.stuff.contains(&[x,y]);
    }
    pub fn get_neighbors(&self, x:i32, y:i32) -> [bool;8]{
        return [
            self.is_alive(x - 1,y - 1),
            self.is_alive(x + 0,y - 1),
            self.is_alive(x + 1,y - 1),
            self.is_alive(x - 1,y + 0),
            //self.is_alive(x + 0,y + 0),
            self.is_alive(x + 1,y + 0),
            self.is_alive(x - 1,y + 1),
            self.is_alive(x + 0,y + 1),
            self.is_alive(x + 1,y + 1),
            ];
    }

    pub fn set_alive(&mut self, x:i32, y:i32) {
       self.stuff.push([x,y]);
    }
}