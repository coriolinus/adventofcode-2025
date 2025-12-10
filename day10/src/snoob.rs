//! Efficiently generate bit permutations under a certain value.
//!
//! I _really_ should consider putting this kind of thing into aoclib.

/// Generate the next highest value having the same number of 1 bits as `x`
///
/// Adapted from https://web.archive.org/web/20130731200134/http://hackersdelight.org/hdcodetxt/snoob.c.txt
fn snoob(x: u32) -> Option<u32> {
    if x == 0 || x == !0 {
        return None;
    }

    let smallest = x & (!x + 1);
    let ripple = x.checked_add(smallest)?;
    let mut ones = x ^ ripple;
    ones >>= 2;
    ones /= smallest;
    Some(ripple | ones)
}

fn n_low_ones(n: u32) -> u32 {
    let x: u32 = !(!0 << n);
    debug_assert_eq!(n, x.count_ones(), "correct number of set bits");
    debug_assert_eq!(x >> n, 0, "correct placement of set bits");
    x
}

#[derive(Debug)]
pub struct PermutationIterator {
    /// how many bits we care about
    width: u32,
    /// how many ones we're currently generating
    n_ones: u32,
    /// the next item to return
    n: u32,
}

impl Iterator for PermutationIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.n;

        self.n = snoob(self.n)?;
        if self.n & !n_low_ones(self.width) != 0 {
            // we overflowed the width we care about
            self.n_ones += 1;
            self.n = n_low_ones(self.n_ones);
        }

        (value < 1 << self.width).then_some(value)
    }
}

impl PermutationIterator {
    /// Create a new permutations iterator.
    ///
    /// Specify the bit width that you care about. This will then iterate over
    /// all permutations of bits which are less than or equal to that width, ordered
    /// first by number of bits set, then by numeric order for a given number of set bits.
    ///
    /// The maximum width is 31. If you need more bits than that, this is probably the wrong
    /// algorithm for AOC.
    pub fn new(width: u32) -> Option<Self> {
        if width == 0 || width > 31 {
            return None;
        }

        Some(Self {
            width,
            n_ones: 1,
            n: 1,
        })
    }
}
