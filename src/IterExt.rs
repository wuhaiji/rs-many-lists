use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::rc::Rc;

pub struct IterExt<ITER>
where
    ITER: Iterator,
{
    iter: Rc<RefCell<ITER>>,
}

impl<ITER> Display for IterExt<ITER>
where
    ITER: Iterator,
    ITER::Item: Display, // Ensure that the items in the iterator can be displayed
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let rc = self.iter.clone();

        let mut ref_mut = rc.borrow_mut();

        while let Some(n) = ref_mut.next() {
            write!(f, "{},", n)?;
        }

        write!(f, "]")
    }
}
