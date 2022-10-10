use std::cell::{RefCell,Cell};

#[derive(Debug)]
struct Computer {
    cpu_name: String,
    owner: RefCell<String>,
    ram_size: Cell<u8>, // 기본 자로형
    is_work: Cell<bool>, // 기본 자료형
}

#[test]
fn main() {
    let com1 = Computer {
        cpu_name: "i5".into(),
        owner: RefCell::new(String::from("keroro")),
        ram_size: Cell::new(4),
        is_work: Cell::new(false),
    };

    println!("{com1:?}");
    com1.is_work.set(true);
    com1.ram_size.set(8);
    let r1_com1 = com1.owner.try_borrow_mut();
    *r1_com1.unwrap() = "kro 0".to_string();
    //drop(r1_com1);
    println!("{com1:?}");
    *com1.owner.borrow_mut() = "kro1".to_string();
    println!("{com1:?}");
}