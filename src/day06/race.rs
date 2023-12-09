#[derive(Debug, Clone)]
pub struct Race {
    time: usize,
    current_record: usize,
}

impl Race {
    pub fn new(time: usize, current_record: usize) -> Self {
        Self {
            time,
            current_record,
        }
    }

    pub fn find_number_of_winning_runs(&self) -> usize {
        let mut minimal_hold_time = 0;
        for hold_time in 0..self.time {
            let distance = (self.time - hold_time) * hold_time;
            if distance > self.current_record {
                minimal_hold_time = hold_time;
                break;
            }
        }

        let mut maximal_hold_time = 0;
        for hold_time in (0..self.time).rev() {
            let distance = (self.time - hold_time) * hold_time;
            if distance > self.current_record {
                maximal_hold_time = hold_time;
                break;
            }
        }
        maximal_hold_time - minimal_hold_time + 1
    }
}
