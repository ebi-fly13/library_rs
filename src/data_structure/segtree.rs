use crate::utility::monoid::Monoid;

pub struct SegTree<M>
where
    M: Monoid,
{
    n: usize,
    size: usize,
    data: Vec<M::S>,
}

impl<M> SegTree<M>
where
    M: Monoid,
{
    pub fn new(n: usize) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        Self {
            n,
            size,
            data: vec![M::e(); 2 * size],
        }
    }

    pub fn set(&mut self, idx: usize, x: M::S) {
        assert!(idx < self.n);
        let mut idx = idx + self.size;
        self.data[idx] = x;
        while idx > 1 {
            idx >>= 1;
            self.data[idx] = M::op(&self.data[idx << 1], &self.data[(idx << 1) | 1]);
        }
    }

    pub fn get(&self, idx: usize) -> M::S {
        self.data[idx + self.size].clone()
    }

    pub fn prod(&self, l: usize, r: usize) -> M::S {
        let mut left = M::e();
        let mut right = M::e();
        let mut l = l + self.size;
        let mut r = r + self.size;
        while l < r {
            if (l & 1) == 1 {
                left = M::op(&left, &self.data[l]);
                l += 1;
            }
            if (r & 1) == 1 {
                r -= 1;
                right = M::op(&self.data[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&left, &right)
    }

    pub fn all_prod(&self) -> M::S {
        self.data[1].clone()
    }
}
