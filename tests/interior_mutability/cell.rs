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
    let com1 = Computer { // mut 변수가 아님 그럼에도 값을 바꿀수 있음
        cpu_name: "i5".into(),
        //owner: Cell::new(String::from("keroro")),
        ram_size: Cell::new(4),
        is_work: Cell::new(false),
    };

    println!("{com1:?} {}",com1.cpu_name);
    com1.is_work.set(true); //Cell::set() 메소드 를 사용하여 값을 바꾼다
    com1.ram_size.set(8); //Cell::set() 메소드 를 사용하여 값을 바꾼다
    println!("{com1:?} {}",com1.cpu_name);
}