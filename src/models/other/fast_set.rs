use std::ops::Index;

// This is for small sets
#[derive(Debug)]
pub struct FastSet<T: Eq> {
    data: Vec<T>,
}

impl<T: Eq> FastSet<T> {
    pub fn new() -> FastSet<T> {
        FastSet { data: Vec::new() }
    }

    pub fn insert(&mut self, v: T) -> usize {
        if let Some(i) = self.data.iter().position(|x| x == &v) {
            i
        } else {
            self.data.push(v);
            self.data.len() - 1
        }
    }

    pub fn contains(&self, v: &T) -> bool { self.data.contains(v) }

    pub fn remove(&mut self, v: &T) -> bool {
        if let Some(i) = self.data.iter().position(|x| x == v) {
            self.data.swap_remove(i);
            return true;
        }
        false
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> { self.data.iter() }

    pub fn len(&self) -> usize { self.data.len() }
}

impl<T: Eq> Index<usize> for FastSet<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}