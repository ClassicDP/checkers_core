use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
pub struct Vector {
    pub(crate) points: Vec<i16>,
    pub(crate) direction_index: i8, // 0..3 (0 - UR, 1 - UL, 2 - DL, 3 - DR): used in Game
}

pub struct VectorIntoIterator {
    vector: Vector,
    index: usize,
}


impl Vector {
    pub fn new(&mut self, direction: i8) {
        self.direction_index = direction;
        self.points = Vec::new();
    }
}

impl Iterator for VectorIntoIterator {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vector.points.len() {
            let i = self.index;
            self.index +=1;
            Some(self.vector.points[i])
        } else { None }
    }
}

impl IntoIterator for Vector {
    type Item = i16;
    type IntoIter = VectorIntoIterator;
    fn into_iter(self) -> Self::IntoIter {
        VectorIntoIterator {
            index: 0,
            vector: self,
        }
    }
}
