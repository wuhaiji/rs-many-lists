use std::collections::linked_list::LinkedList;

trait Collection<T> {
    type Iter<'iter>: Iterator<Item=&'iter T>
        where
            Self: 'iter,  // 此行的添加是由于 https://github.com/rust-lang/rust/issues/87479 中的限制
            T: 'iter;
    // T 的生命周期需要 比 'iter 更久才能借用
    
    fn empty() -> Self;
    fn add(&mut self, value: T);
    
    // fn iterate(&'iter self) -> Self::Iter<'iter>;
    fn iterate(&self) -> Self::Iter<'_>;
}


impl<T> Collection<T> for Vec<T> {
    type Iter<'iter> = std::slice::Iter<'iter, T> where T: 'iter;
    
    fn empty() -> Self {
        vec![]
    }
    
    fn add(&mut self, value: T) {
        self.push(value)
    }
    
    fn iterate(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

impl<T> Collection<T> for LinkedList<T> {
    type Iter<'iter> = std::collections::linked_list::Iter<'iter, T> where T: 'iter;
    
    fn empty() -> Self {
        LinkedList::new()
    }
    
    fn add(&mut self, value: T) {
        self.push_back(value);
    }
    
    fn iterate(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

fn floatify<Input, Output>(ints: &Input) -> Output
    where
        Input: Collection<i32>,
        Output: Collection<f32>
{
    let mut floats = Output::empty();
    for &i in ints.iterate() {
        floats.add(i as f32);
    }
    floats
}


fn main() {
    let v: Vec<_> = vec![1, 2, 3];
    let v: Vec<_> = floatify(&v);
    
    let l: LinkedList<_> = LinkedList::from_iter([4, 5, 6]);
    let l: LinkedList<_> = floatify(&l);
}