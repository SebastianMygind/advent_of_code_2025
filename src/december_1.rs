use crate::december_1::dial::Dial;

pub mod dial;
pub mod input_handler;
pub mod input_iter;

pub fn challenge() {
    let input = input_handler::read_input();
    let mut dial = Dial::default();

    let mut zero_count = 0;

    for (direction, rotations) in input {
        let zero_ticks = dial.rotate(direction, rotations);

        zero_count += zero_ticks
    }

    println!("The amount of times the pointer was at zero: {zero_count}");
}
