#[derive(Default, Debug)]
pub struct NaturalNumbers {
    current: u32,
    multiples: Vec<u32>,
}

impl NaturalNumbers {
    pub fn new() -> Self {
        Self {
            current: 0,
            multiples: vec![],
        }
    }

    pub fn set_multiples(&self, multiples: Vec<u32>) -> NaturalNumbers {
        Self { multiples, ..*self }
    }
}

impl Iterator for NaturalNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current += 1;
            for multiple in &self.multiples {
                if self.current % multiple == 0 {
                    return Some(self.current);
                }
            }
        }
    }
}
