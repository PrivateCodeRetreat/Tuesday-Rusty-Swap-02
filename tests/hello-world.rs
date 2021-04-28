#![cfg_attr(feature = "strict", deny(warnings))]

use hello_world::Board;

#[test]
fn die_from_underpopulation() {
    assert_eq!(false, hello_world::next_state(true, 1));
}

#[test]
fn stay_alive() {
    assert_eq!(true, hello_world::next_state(true, 2));
    assert_eq!(true, hello_world::next_state(true, 3));
}

#[test]
fn any_live_cell_with_more_than_three_live_neighbours_dies() {
    assert_eq!(false, hello_world::next_state(true, 4));
}

#[test]
fn dead_cell_with_exactly_three_live_neighbours_live() {
    assert_eq!(true, hello_world::next_state(false, 3));
}

#[test]
fn dead_cell_with_two_live_neighbours_stays_dead() {
    assert_eq!(false, hello_world::next_state(false, 2));
}
#[test]
fn calculate_live_neighbours_count() {
    let neighbors = [true, false, true, false, true, false, false, false];
    assert_eq!(3, hello_world::calculate_live_neighbors(neighbors));
}
#[test]
fn calculate_live_neighbours_count_4() {
    let neighbors = [true, true, true, false, true, false, false, false];
    assert_eq!(4, hello_world::calculate_live_neighbors(neighbors));
}

#[test]
fn get_neighbors_from_board() {
    let board = Board::new();
    assert_eq!([false,false,false,false,false,false,false,false], board.get_neighbors(1,1))
}


#[test]
fn get_neighbors_from_board_with_stuff() {
    let mut board = Board::new();
    board.set_alive(0,0);
    assert_eq!([true,false,false,false,false,false,false,false], board.get_neighbors(1,1))
}
#[test]
fn get_neighbors_from_board_with_different_stuff() {
    let mut board = Board::new();
    board.set_alive(1,0);
    assert_eq!([false,true,false,false,false,false,false,false], board.get_neighbors(1,1))
}