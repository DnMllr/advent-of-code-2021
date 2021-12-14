use std::collections::HashMap;

use crate::parser::Input;

pub struct Replacer<'a> {
    input: &'a Input<'a>,
    frequencies: HashMap<u8, usize>,
    from: HashMap<(u8, u8), usize>,
    to: HashMap<(u8, u8), usize>,
}

impl<'a> std::fmt::Display for Replacer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for ((l, r), count) in self.from.iter() {
            let slice = &[*l, *r][..];
            let key = unsafe { std::str::from_utf8_unchecked(slice) };
            writeln!(f, "{}: {}", key, count)?;
        }
        Ok(())
    }
}

impl<'a> Replacer<'a> {
    pub fn new(input: &'a Input<'a>) -> Self {
        let mut from = HashMap::new();
        let mut frequencies = HashMap::new();

        for window in input.polymer.windows(2) {
            *from.entry((window[0], window[1])).or_default() += 1;
        }

        for byte in input.polymer {
            *frequencies.entry(*byte).or_default() += 1;
        }

        Self {
            input,
            from,
            frequencies,
            to: HashMap::new(),
        }
    }

    pub fn apply(&mut self) {
        for ((l, r), count) in self.from.drain() {
            if let Some(byte) = self.input.replacement_rule.get(&(l, r)) {
                *self.to.entry((l, *byte)).or_default() += count;
                *self.to.entry((*byte, r)).or_default() += count;
                *self.frequencies.entry(*byte).or_default() += count;
            }
        }

        std::mem::swap(&mut self.from, &mut self.to);
    }

    pub fn frequency_diff(&self) -> usize {
        self.frequencies
            .values()
            .max()
            .zip(self.frequencies.values().min())
            .map(|(max, min)| max - min)
            .expect("there will be a polymer")
    }

    pub fn apply_n(&mut self, n: usize) {
        (0..n).for_each(|_| self.apply());
    }
}
