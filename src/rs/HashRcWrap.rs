use std::fmt::Debug;
use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HashRcWrap<T> {
    value: Rc<RefCell<T>>,
}

impl<T> Deref for HashRcWrap<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        self.value.deref()
    }
}

impl<T: Debug> HashRcWrap<T> {
    pub fn new(value: T) -> HashRcWrap<T> {
        HashRcWrap {
            value: Rc::new(RefCell::new(value)),
        }
    }
    pub fn get_unwrap_mut(&self) -> RefMut<'_, T> {
        self.value
            .deref()
            .try_borrow_mut()
            .expect("already borrowed")
    }
    pub fn get_unwrap(&self) -> Ref<'_, T> {
        self.value.deref().borrow()
    }
}

impl<T> PartialEq<Self> for HashRcWrap<T> {
    fn eq(&self, other: &Self) -> bool {
        (*self.value).as_ptr() == (*other.value).as_ptr()
    }
}


impl<T> Hash for HashRcWrap<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = (*self.value).as_ptr();
        ptr.hash(state)
    }
}
