use crate::brick_stack::BrickStack;

mod brick_stack;

const INPUT: &str = include_str!("../../data/day22/input.txt");

fn main() {
    let mut stack = BrickStack::new(INPUT);
    stack.move_bricks();

    println!("result: {:?}", stack.get_disintegratable_bricks().count());
}
