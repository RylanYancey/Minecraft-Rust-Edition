
/// An iterator over the indices of 1's in a bitfield.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BitIterU8(pub u8);

impl Iterator for BitIterU8 {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let tz = self.0.trailing_zeros() as usize;
            self.0 &= self.0 - 1;
            Some(tz)
        }
    }
}

#[cfg(test)]
mod test {
    use super::BitIterU8;
    
    #[test]
    fn bit_iter() {
        assert_eq!(vec![1, 3, 4, 6], BitIterU8(0b0101_1010).collect::<Vec<usize>>());
    }
}
