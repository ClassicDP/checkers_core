use std::cmp::min;

pub struct MutIter<'a, T> {
    v: &'a Vec<T>,
    index: usize,
    range_a: Option<usize>,
    range_b: Option<usize>,
    len_or_range_b: usize,
}

impl<'a, T> IntoIterator for &'a MutIter<'a, T> {
    type Item = &'a T;
    type IntoIter = MutIter<'a, T>;

    fn into_iter(self) -> MutIter<'a, T> {
        MutIter::new(self.v, self.range_a, self.range_b)
    }
}

impl<'a, T> Iterator for MutIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len_or_range_b {
            self.index += 1;
            return Some(&self.v[self.index - 1]);
        }
        None
    }
}

impl<'a, T> MutIter<'a, T> {
    fn new(v: &'a Vec<T>, range_a: Option<usize>, range_b: Option<usize>) -> MutIter<'a, T> {
        let index = if range_a.is_some() { range_a.unwrap() } else { 0 };
        let len = if range_b.is_some() { min(range_b.unwrap(), v.len()) } else { v.len() };
        MutIter {
            v,
            index,
            range_a,
            range_b,
            len_or_range_b: len,
        }
    }
}

#[derive(Clone, Debug)]
struct St0 {
    a: i32,
    b: i32,
}

#[cfg(test)]
mod tests {
    use std::cell::{RefCell};
    use std::ptr;
    use std::rc::Rc;
    use crate::mutable_iterator::{MutIter, St0};

    #[test]
    fn vector() {
        let st = RefCell::new(St0 { a: 0, b: 0 });
        let v = vec![Rc::new(st.clone()), Rc::new(st.clone()),
                     Rc::new(st.clone())];
        let i1 = MutIter::new(&v, None, Some(3));
        let mut v2: Vec<Rc<RefCell<St0>>> = Vec::new();
        for i in &i1 {
            (**i).borrow_mut().b += 1;
            v2.push(i.clone());
        }
        for (i, it) in i1.enumerate() {
            (**it).borrow_mut().a = i as i32;
            let x = (**it).as_ptr();
            let y = v2[i].as_ptr();
            assert!(ptr::eq(x, y));
            assert_eq!(x, y);
        }
    }
}
