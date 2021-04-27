#![cfg_attr(feature = "strict", deny(warnings))]

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
