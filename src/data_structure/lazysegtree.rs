use crate::utility::monoid::MapMonoid;

pub struct LazySegTree<M>
where
    M: MapMonoid, 
{
    n: usize,
    size: usize,
    data: Vec<M::S>,
    lazy: Vec<M::F>,
}
