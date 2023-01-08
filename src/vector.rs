use std::slice::IterMut;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
#[derive(Clone, Serialize, Deserialize, Debug)]
#[derive(TS)]
pub struct Vector<T> {
    pub(crate) points: Vec<T>,
    pub(crate) direction_index: i8, // 0..3 (0 - UR, 1 - UL, 2 - DL, 3 - DR): used in Game
}

pub struct VectorIntoIterator<'a, T> {
    vector: &'a Vector<T>,
    index: usize,
}


impl <T> Vector<T> {
    pub fn new(direction: i8) -> Vector<T> {
        Vector {
            points: Vec::new(),
            direction_index: direction,
        }
    }
}

impl <'a, T> Iterator for VectorIntoIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vector.points.len() {
            let i = self.index;
            self.index +=1;
            Some(&self.vector.points[i])
        } else { None }
    }
}



impl <'a, T> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = VectorIntoIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        VectorIntoIterator {
            index: 0,
            vector: &self,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::{Borrow, BorrowMut};
    use std::ptr;
    use std::ptr::eq;
    use crate::vector::Vector;
    #[test]
    fn vector() {
        let mut v = Vector::new(0);
        v.points = vec![0,1,2,3,4,5];
        let mut i=0;
        for p in &v {
            print!("{}",p);
            assert!(eq(p, &v.points[i]));
            i+=1;
        }
    }
}
