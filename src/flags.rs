#[derive(Copy, Clone, Debug)]
pub struct Flags(pub u64);

impl Flags {
    #[inline]
    pub const fn get_bit(&self, i: usize) -> bool {
        self.0 & (1 << i) != 0
    }

    #[inline]
    pub fn set_bit(&mut self, i: usize, value: bool) {
        if value {
            self.0 |= 1 << i;
        } else {
            self.0 &= !(1 << i);
        }
    }

    #[inline]
    pub fn toggle_bit(&mut self, i: usize) {
        self.set_bit(i, !self.get_bit(i));
    }
}