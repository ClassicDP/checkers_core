use serde::{Deserialize, Serialize};
use std::rc::Rc;
use ts_rs::TS;

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
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

    pub fn get_ban_direction(&self) -> i8 {
        (self.direction + 2) % 4
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
        } else {
            None
        }
    }
}

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

impl<'a, T> IntoIterator for &'a Vector<T> {
    type Item = &'a T;
    type IntoIter = VectorIntoIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        let a = if self.range_a.is_some() {
            min(self.range_a.unwrap(), self.points.len())
        } else {
            0
        };
        let b = if self.range_b.is_some() {
            min(self.range_b.unwrap(), self.points.len())
        } else {
            self.points.len()
        };
        VectorIntoIterator {
            index: a,
            vector: &self,
            range_b: b,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;
    use std::ptr::eq;

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
