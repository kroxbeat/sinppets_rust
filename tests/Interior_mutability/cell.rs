use std::cell::Cell;

#[derive(Debug)]
struct Computer {
    cpu_name: String,
    //owner: Cell<String>, // 참조형 이 가능하려면 Copy Trait 구현 되 어야함
    ram_size: Cell<u8>, // 기본 자로형
    is_work: Cell<bool>, // 기본 자료형
}

#[test]
fn main() {
    let com1 = Computer {
        cpu_name: "i5".into(),
        //owner: Cell::new(String::from("keroro")),
        ram_size: Cell::new(4),
        is_work: Cell::new(false),
    };

    println!("{com1:?}");
    com1.is_work.set(true);
    com1.ram_size.set(8);
    println!("{com1:?}");
}