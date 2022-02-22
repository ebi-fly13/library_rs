use crate::utility::monoid::{MapMonoid, Monoid};

pub struct LazySegTree<M>
where
    M: MapMonoid,
{
    n: usize,
    size: usize,
    data: Vec<<M::S as Monoid>::S>,
    lazy: Vec<M::F>,
}

impl<M> LazySegTree<M>
where
    M: MapMonoid,
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
            lazy: vec![M::id(); size],
        }
    }

    pub fn set(&mut self, idx: usize, x: <M::S as Monoid>::S) {}

    pub fn get(&self, idx: usize) -> <M::S as Monoid>::S {
        M::e()
    }

    pub fn prod(&self, l: usize, r: usize) -> <M::S as Monoid>::S {
        M::e()
    }

    pub fn all_prod(&self) -> <M::S as Monoid>::S {
        M::e()
    }

    pub fn apply(&mut self, l: usize, r: usize, f: M::F) {}
}

impl<M> LazySegTree<M>
where
    M: MapMonoid,
{
    fn update(&mut self, idx: usize) {}

    fn all_apply(&mut self, idx: usize) {}

    fn push(&mut self, idx: usize) {}
}
