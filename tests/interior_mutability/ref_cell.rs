use std::cell::{RefCell};

#[derive(Debug)]
struct Computer {
    name: RefCell<String>,
    price: RefCell<u32>,
    // 참조형 내부변동성 RefCell::replace 이용시 자동Drop 됨 , borrow 이용시 는 소유권 주의 ! ( 동시에 2개 생성 불가 , borrow 중에 출력시 값이 아닌 'borrowed' 출력 됨 )
}

#[test]
fn main() {
    let com1 = Computer {
        name: RefCell::new(String::from("keroro")),
        price: RefCell::new(100),
    };

    println!("{com1:?} p 1");

    // 가변 빌림 시도
    let _owner_0 = com1.name.try_borrow_mut();
    println!("{:?} p 2",_owner_0);
    *_owner_0.unwrap() = "abcc".into(); // unwrap 이후 drop 으로 다시 빌림 가능 해짐
    println!("{com1:?} p 1.5");

    com1.name.replace("ppppp".into()); //replace 이용시 자동Drop 됨

    // 가변 빌림 시도 1
    let _price_0 = com1.price.try_borrow_mut();
    println!("{:?} p 3",_price_0);
    _price_0.unwrap(); // 이후 자동 drop 으로 다시 빌림 가능 해짐
    // 가변 빌림 시도 2
    let _price_1 = com1.price.try_borrow_mut(); // 빌려 놓고 반납 안하면 'borrowed' 로 출력
    println!("{:?} p 4",_price_1);


    println!("{com1:?} p 2");

}