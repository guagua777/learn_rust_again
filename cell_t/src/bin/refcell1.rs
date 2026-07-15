use std::rc::Rc;
use std::cell::RefCell;


// fn forget<T>(val: T) { 
//     let x = Rc::new((val, RefCell::new(None))); 
//     // *x.1.borrow_mut() = x.clone();
//     // *x.1.borrow_mut() = Some(1);
//     *x.1.borrow_mut() = Some(x.clone());
//     // Rc::clone(&self)
// }



fn forget<T>(val: T) {
    use std::cell::RefCell;
    use std::rc::Rc;
    struct Foo<T>(T, RefCell<Option<Rc<Foo<T>>>>);
    let x = Rc::new(Foo(val, RefCell::new(None)));
    *x.1.borrow_mut() = Some(x.clone());
}



// fn forget1<T>(val: T) {
//     use std::cell::RefCell;
//     use std::rc::Rc;
//     struct Foo<T>(T, RefCell<Option<Box<Foo<T>>>>);
//     let x = Box::new(Foo(val, RefCell::new(None)));
//     *x.1.borrow_mut() = Some(x);
// }


struct DontDropMe;
impl Drop for DontDropMe {
    fn drop(&mut self) { unreachable!() }
}
fn main() {
    forget(DontDropMe)
}