use std::rc::Rc;
use std::slice::IterMut;
use js_sys::Math::min;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(TS)]
pub struct Vector<T> {
    pub(crate) points: Rc<Vec<T>>,
    pub(crate) direction: i8,
    // 0..3 (0 - UR, 1 - UL, 2 - DL, 3 - DR): used in Game
    range_a: Option<usize>,
    range_b: Option<usize>,
}

pub struct VectorIntoIterator<'a, T> {
    vector: &'a Vector<T>,
    index: usize,
    range_b: usize,
}



impl<T> Vector<T> {
    pub fn new(direction: i8, points: Vec<T>) -> Vector<T> {
        Vector {
            points: Rc::new(points),
            direction,
            range_a: None,
            range_b: None,
        }
    }

    pub fn set_range(&mut self, a: usize, b: usize) {
        self.range_a = Some(a);
        self.range_b = Some(b);
    }
    pub fn clear_range(&mut self) {
        self.range_a = None;
        self.range_b = None;
    }
}

impl<'a, T> Iterator for VectorIntoIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.range_b {
            let i = self.index;
            self.index += 1;
            Some(&self.vector.points[i])
        } else { None }
    }
}


impl<'a, T> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = VectorIntoIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        let a = if self.range_a.is_some() {
            min(self.range_a.unwrap() as f64,
                self.points.len() as f64)
        } else {
            0f64
        };
        let b = if self.range_b.is_some() {
            min(self.range_b.unwrap() as f64,
                self.points.len() as f64)
        } else {
            self.points.len() as f64
        };
        VectorIntoIterator {
            index: a as usize,
            vector: &self,
            range_b: b as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::eq;
    use crate::vector::Vector;

    #[test]
    fn vector() {
        let v = Vector::new(0, vec![0, 1, 2, 3, 4, 5]);
        let v1 = v.clone();
        for (i, p) in v1.into_iter().enumerate() {
            print!(" {}, {} ", p, i);
            assert!(eq(p, &v.points[i]));
        }
    }
}
