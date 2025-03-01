/*
** src/puzzles/aoc2022/day6.rs
*/

use super::Solution;

const N_CHARS: usize = 26;
const CHAR_BASE: u32 = 'a' as u32;

const PACKET_MARKER_SIZE: usize = 4;
const MESSAGE_MARKER_SIZE: usize = 14;

const fn char_index(c: char) -> usize {
    ((c as u32) - CHAR_BASE) as usize
}

struct UniqueCharCounter {
    counts: [u32; N_CHARS],
}

impl UniqueCharCounter {
    fn new() -> Self {
        let counts = [0; N_CHARS];
        Self { counts }
    }

    fn add(&mut self, c: char) {
        let i = char_index(c);
        self.counts[i] += 1;
    }

    fn remove(&mut self, c: char) {
        let i = char_index(c);
        self.counts[i] -= 1;
    }

    fn all_unique(&self) -> bool {
        // if all counts are 0 or 1, no bits higher than the lowest will be set
        // once all counts are or-ed together
        let mash = self
            .counts
            .into_iter()
            .reduce(|acc, cnt| acc | cnt)
            .unwrap();
        (mash & (!0x1)) == 0
    }
}

pub fn solve(input: String) -> Solution {
    let mut solution = Solution::new();
    // split input into an array of characters
    let stream = input.chars().collect::<Vec<_>>();
    let size = stream.len();
    // initialize counters for start-of-packet and start-of-message searches
    let mut packet_char_counter = UniqueCharCounter::new();
    let mut message_char_counter = UniqueCharCounter::new();

    // Part A: How many characters need to be processed before the first start-of-packet marker
    // is detected?
    // initialize with the first characters
    for c in &stream[..PACKET_MARKER_SIZE] {
        packet_char_counter.add(*c);
    }
    // then use a sliding window to find the start-of-packet marker
    let mut wi = 0;
    let mut wj = PACKET_MARKER_SIZE;
    while wj < size && !packet_char_counter.all_unique() {
        // add the next character to the window and remove the character from
        // the start of the old window
        packet_char_counter.remove(stream[wi]);
        packet_char_counter.add(stream[wj]);
        wi += 1;
        wj += 1;
    }
    let packet_start = if wj == size { None } else { Some(wj) };
    solution.maybe_set_part_a(packet_start);

    // Part B: How many characters need to be processed before the first start-of-message marker
    // is detected?
    // initialize with the first characters
    for c in &stream[..MESSAGE_MARKER_SIZE] {
        message_char_counter.add(*c);
    }
    // then use a sliding window to find the start-of-packet marker
    let mut wi = 0;
    let mut wj = MESSAGE_MARKER_SIZE;
    while wj < size && !message_char_counter.all_unique() {
        // add the next character to the window and remove the character from
        // the start of the old window
        message_char_counter.remove(stream[wi]);
        message_char_counter.add(stream[wj]);
        wi += 1;
        wj += 1;
    }
    let message_start = if wj == size { None } else { Some(wj) };
    solution.maybe_set_part_b(message_start);

    solution
}
