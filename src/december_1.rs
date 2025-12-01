use crate::december_1::dial::Dial;

pub mod dial;
pub mod input_handler;
pub mod input_iter;

pub fn challenge() {
    let input = input_handler::read_input();
    let mut dial = Dial::default();

    let mut zero_count = 0;

    for (direction, rotations) in input {
        let new_position = dial.rotate(direction, rotations);

        if new_position == 0 {
            zero_count += 1;
        }
    }

    println!("The amount of times landed on zero: {zero_count}");
}
