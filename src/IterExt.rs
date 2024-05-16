use std::fmt::Debug;

pub struct IterExt<I>(I) where I: Iterator;

impl<I> IterExt<I>
    where
        I: Iterator,
{
    pub fn new(iter: I) -> Self {
        IterExt(iter)
    }
    
    pub fn print_all(&mut self)
        where
            I::Item: Debug,
    {
        for item in &mut self.0 {
            println!("{:?}", item);
        }
    }
}