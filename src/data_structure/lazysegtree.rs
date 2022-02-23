use crate::utility::monoid::{MapMonoid, Monoid};

pub struct LazySegTree<M>
where
    M: MapMonoid,
{
    n: usize,
    size: usize,
    log: usize,
    data: Vec<<M::S as Monoid>::S>,
    lazy: Vec<M::F>,
}

impl<M> LazySegTree<M>
where
    M: MapMonoid,
{
    pub fn new(n: usize) -> Self {
        let mut log = 1;
        while (1 << log) < n {
            log += 1;
        }
        let size = 1 << log;
        Self {
            n,
            size,
            log,
            data: vec![M::e(); 2 * size],
            lazy: vec![M::id(); size],
        }
    }

    pub fn set(&mut self, idx: usize, x: <M::S as Monoid>::S) {
        assert!(idx < self.n);
        let idx = idx + self.size;
        for i in (1..(self.log + 1)).rev() {
            self.push(idx >> i);
        }
        self.data[idx] = x;
        for i in 1..(self.log + 1) {
            self.update(idx >> i);
        }
    }

    pub fn get(&mut self, idx: usize) -> <M::S as Monoid>::S {
        assert!(idx < self.n);
        let idx = idx + self.size;
        for i in (1..(self.log + 1)).rev() {
            self.push(idx >> i);
        }
        self.data[idx].clone()
    }

    pub fn prod(&mut self, l: usize, r: usize) -> <M::S as Monoid>::S {
        assert!(l < self.n && r <= self.n);
        if l == r {
            return M::e();
        }
        let l = l + self.size;
        let r = r + self.size;
        for i in (1..(self.log + 1)).rev() {
            if (l >> i) << i != l {
                self.push(l >> i);
            }
            if (r >> i) << i != r {
                self.push((r - 1) >> i);
            }
        }
        let mut l = l;
        let mut r = r;
        let mut left = M::e();
        let mut right = M::e();
        while l < r {
            if l & 1 == 1 {
                left = M::op(&left, &self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                right = M::op(&self.data[r], &right);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&left, &right)
    }

    pub fn all_prod(&self) -> <M::S as Monoid>::S {
        self.data[1].clone()
    }

    pub fn apply(&mut self, l: usize, r: usize, f: M::F) {
        assert!(l < self.n && r <= self.n);
        if l == r {
            return;
        }
        let mut l = l + self.size;
        let mut r = r + self.size;
        for i in (1..(self.log + 1)).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        {
            let keep = (l, r);
            while l < r {
                if l & 1 == 1 {
                    self.all_apply(l, f.clone());
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    self.all_apply(r, f.clone());
                }
                l >>= 1;
                r >>= 1;
            }
            l = keep.0;
            r = keep.1;
        }
        for i in (1..(self.log + 1)).rev() {
            if (l >> i) << i != l {
                self.update(l >> i);
            }
            if (r >> i) << i != r {
                self.update((r - 1) >> i);
            }
        }
    }
}

impl<M> LazySegTree<M>
where
    M: MapMonoid,
{
    fn update(&mut self, idx: usize) {
        assert!(idx < self.size);
        self.data[idx] = M::op(&self.data[2 * idx], &self.data[2 * idx + 1]);
    }

    fn all_apply(&mut self, idx: usize, f: M::F) {
        assert!(idx < 2 * self.size);
        self.data[idx] = M::mapping(&f, &self.data[idx]);
        if idx < self.size {
            self.lazy[idx] = M::composition(&f, &self.lazy[idx]);
        }
    }

    fn push(&mut self, idx: usize) {
        assert!(idx < self.size);
        self.all_apply(idx << 1, self.lazy[idx].clone());
        self.all_apply((idx << 1) + 1, self.lazy[idx].clone());
        self.lazy[idx] = M::id();
    }
}
