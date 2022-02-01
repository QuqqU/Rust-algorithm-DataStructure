mod bit {
    use std::ops::{Add, Sub};

    ///
    /// Usage:
    /// let mut tree = bit::BIT::<i32>::new(n as usize);
    /// tree.update(idx, value);
    /// tree.query(idx); // sum of from 1 to idx
    /// tree.sum(l, r); // sum of from l to r
    /// 

    #[derive(Debug)]
    pub struct BIT<T> {
        tree: Vec<T>,
        size: usize,
    }

    impl<T> BIT<T>
    where
        T: Copy + Default + Add<Output = T> + Sub<Output = T>,
    {
        pub fn new(size: usize) -> Self {
            assert!(0 < size);
            Self {
                tree: vec![T::default(); size + 1],
                size,
            }
        }

        pub fn update(&mut self, idx: usize, value: T) {
            assert!(0 < idx && idx <= self.size);
            let mut idx = idx as i32;
            while idx <= self.size as i32 {
                self.tree[idx as usize] = self.tree[idx as usize] + value;
                idx += (idx & -idx);
            }
        }

        pub fn query(&self, idx: usize) -> T {
            assert!(idx <= self.size);
            let mut ans: T = T::default();
            let mut idx = idx as i32;
            while idx > 0 {
                ans = ans + self.tree[idx as usize];
                idx -= (idx & -idx);
            }
            ans
        }

        pub fn sum(&self, from: usize, to: usize) -> T {
            self.query(to) - self.query(from - 1)
        }
    }
}
