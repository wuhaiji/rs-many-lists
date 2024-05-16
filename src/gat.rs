use std::rc::Rc;
use std::sync::Arc;

trait RefCountedFamily {
    type Pointer<T>;
    fn new<T>(value: T) -> Self::Pointer<T>;
    // ...
    // ...
}

struct RcFamily;

impl RefCountedFamily for RcFamily {
    type Pointer<T> = Rc<T>;
    fn new<T>(value: T) -> Self::Pointer<T> {
        Rc::new(value)
    }
   
}

struct ArcFamily;

impl RefCountedFamily for ArcFamily {
    type Pointer<T> = Arc<T>;
    fn new<T>(value: T) -> Self::Pointer<T> {
        Arc::new(value)
    }

}

struct Container<P: RefCountedFamily, T> {
    data: P::Pointer<T>,
}

impl<P: RefCountedFamily, T> Container<P, T> {
    fn new(v: T) -> Self {
        Container { data: P::new(v) }
    }
}

#[test]
fn t(){
    let c = Container::<RcFamily, &str>::new("123");
}