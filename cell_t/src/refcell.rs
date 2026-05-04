use std::cell::RefCell;




#[test]
fn main5() {
    let rc = RefCell::new(5);

    {
        let data = rc.borrow();
        let data2 = rc.borrow();
    }

    let mut data3 = rc.borrow_mut();
    *data3 = 13;

    let v = rc.try_borrow();
    let v_err = v.is_err();
    println!("v_err is: {v_err:#?}");
    assert!(v_err);

    println!("{rc:#?}");
}



#[test]
fn main4() {
    let rc = RefCell::new(5);

    {
        let data = rc.borrow();
        let data2 = rc.borrow();
    }

    // let mut data3 = rc.borrow_mut();
    // *data3 = 13;
    // drop(data3);

    *rc.borrow_mut() = 100;

    println!("{rc:#?}");
}

#[test]
fn main3() {
    let rc = RefCell::new(5);

    {
        let data = rc.borrow();
        let data2 = rc.borrow();
    }
    let data3 = rc.borrow_mut();

    println!("{rc:#?}");
}

#[test]
fn main2() {
    let rc = RefCell::new(5);

    let data = rc.borrow();
    let data2 = rc.borrow();

    let data3 = rc.borrow_mut();

    println!("{rc:#?}");
}

#[test]
fn main1() {
    let rc = RefCell::new(5);

    let data = rc.borrow();
    let data2 = rc.borrow();

    println!("{rc:#?}");
}
