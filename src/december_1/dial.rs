#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Dial {
    pointer: u32,
}

impl Default for Dial {
    fn default() -> Self {
        Self { pointer: 50 }
    }
}

impl Dial {
    /*
     * This function rotates the dial the given distance and direction, returning the point we end at.
     */
    pub fn rotate(&mut self, direction: Direction, distance: u32) -> u32 {
        let mut remainder = distance;
        let mut zero_ticks = 0;

        match direction {
            Direction::Left => {
                while remainder > 0 {
                    if self.pointer == 0 {
                        zero_ticks += 1;
                    }

                    if self.pointer == 0 {
                        remainder -= 1;
                        self.pointer = 99;
                    } else {
                        remainder -= 1;
                        self.pointer -= 1;
                    }
                }
            }
            Direction::Right => {
                while remainder > 0 {
                    if self.pointer == 0 {
                        zero_ticks += 1;
                    }

                    if self.pointer == 99 {
                        remainder -= 1;
                        self.pointer = 0;
                    } else {
                        remainder -= 1;
                        self.pointer += 1;
                    }
                }
            }
        }
        return zero_ticks;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dial_turn_right_1() {
        let mut dial = Dial::default();

        let res = dial.rotate(Direction::Right, 5);

        assert_eq!(res, 55);
    }

    #[test]
    fn dial_turn_right_2() {
        let mut dial = Dial::default();

        let res = dial.rotate(Direction::Right, 50);

        assert_eq!(res, 0);
    }

    #[test]
    fn dial_turn_left_1() {
        let mut dial = Dial::default();

        let res = dial.rotate(Direction::Left, 5);

        assert_eq!(res, 45);
    }

    #[test]
    fn dial_turn_left_2() {
        let mut dial = Dial::default();

        let res = dial.rotate(Direction::Left, 50);

        assert_eq!(res, 0);
    }
}
