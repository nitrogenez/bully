use rand::{thread_rng, Rng};

#[derive(Debug, Default)]
pub struct PinGenerator {
    pub length: usize,
    pub combination: Vec<u8>,
    pub adb_keycodes: Vec<u8>,
}

impl PinGenerator {
    pub fn new(pin_length: usize) -> PinGenerator {
        PinGenerator {
            length: pin_length,
            combination: vec![0; pin_length],
            adb_keycodes: vec![0; pin_length],
        }
    }

    pub fn generate(&mut self) {
        for i in 0..=self.length - 1 {
            self.combination[i] = thread_rng().gen_range(0..=9);
        }
        self.adb_keycodes = self.combination.clone();

        for i in 0..=self.length - 1 {
            self.adb_keycodes[i] += 7;
        }
    }
}
