mod segment_tree {

    // base-0

    use core::panic;
    use std::fmt::Debug;
    use std::iter::FromIterator;
    use std::ops::{
        Bound::{self, *},
        RangeBounds,
    };
    use std::ptr;

    pub trait Monoid: Clone + Debug {
        const DEFAULT: Self;
        fn opr(&self, rhs: &Self) -> Self;
    }

    #[derive(Debug)]
    pub struct SegmentTree<M: Monoid> {
        tree: Vec<M>,
        size: usize,
        base: usize,
    }

    impl<M: Monoid> SegmentTree<M> {
        pub fn new(size: usize) -> Self {
            let mut base = size.next_power_of_two();
            Self {
                tree: vec![M::DEFAULT; base * 2],
                size,
                base,
            }
        }

        pub fn update(&mut self, idx: usize, value: M) {
            assert!((0..self.size).contains(&idx));
            self.__update(idx, value, 1, 0, self.base - 1);
        }

        fn __update(&mut self, idx: usize, value: M, node: usize, nl: usize, nr: usize) -> M {
            if idx < nl || nr < idx {
                return self.tree[node].clone();
            }
            if nl == nr {
                self.tree[node] = value;
                return self.tree[node].clone();
            }

            self.tree[node] = self
                .__update(idx, value.clone(), node * 2, nl, (nl + nr) / 2)
                .opr(&self.__update(idx, value, node * 2 + 1, (nl + nr) / 2 + 1, nr));

            self.tree[node].clone()
        }

        pub fn query<R: RangeBounds<usize>>(&mut self, range: R) -> M {
            let l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => *l + 1,
                Bound::Unbounded => 0,
            };
            let r = match range.end_bound() {
                Bound::Included(r) => *r,
                Bound::Excluded(r) => *r - 1,
                Bound::Unbounded => self.size,
            };

            assert!((0..self.size).contains(&l) && (0..self.size).contains(&r));

            self.__query(l, r, 1, 0, self.base - 1)
        }

        fn __query(&mut self, l: usize, r: usize, node: usize, nl: usize, nr: usize) -> M {
            if r < nl || nr < l {
                return M::DEFAULT;
            }
            if l <= nl && nr <= r {
                return self.tree[node].clone();
            }
            self.__query(l, r, node * 2, nl, (nl + nr) / 2)
                .opr(&self.__query(l, r, node * 2 + 1, (nl + nr) / 2 + 1, nr))
        }
    }

    impl<M: Monoid> FromIterator<M> for SegmentTree<M> {
        fn from_iter<I: IntoIterator<Item = M>>(iter: I) -> Self {
            let mut iter = iter.into_iter();
            let (_, upper) = iter.size_hint();

            if let Some(size) = upper {
                let base = size.next_power_of_two();
                let mut tree = Vec::with_capacity(base * 2);
                let tree_ptr: *mut M = tree.as_mut_ptr();

                unsafe {
                    tree.set_len(base * 2);

                    for i in 0..base {
                        let value = if let Some(m) = iter.next() {
                            m
                        }
                        else {
                            M::DEFAULT
                        };
                        ptr::write(tree_ptr.add(base + i), value);
                    }

                    for i in (1..base).rev() {
                        ptr::write(tree_ptr.add(i), tree[i * 2].opr(&tree[i * 2 + 1]));
                    }
                }

                SegmentTree { tree, size, base }
            }
            else {
                panic!("iterator upperbound not closed");
            }
        }
    }
}
