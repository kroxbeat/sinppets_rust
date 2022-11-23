/*
    trait DoA 공유 동작 예제
*/

#[allow(dead_code)]
struct MyBox<T> {
    data: T,
}

impl<T> DoA for MyBox<T> {
    fn do_b(&self) {
        println!("do_b - MyBox ");
    }
}

struct YourBox {
    y_data: String,
}

impl DoA for YourBox {
    fn do_b(&self) {
        println!("do_b - YourBox {}", self.y_data);
    }

    fn do_c(&self) {
        println!("called do_c YourBox"); // override 정의
    }
}

//DoA 를 구현한 자료형을 받는 함수 정의
fn hello<T>(input: T)
    where T: DoA
{
    input.do_b();
    input.do_c();
}

//공유 동작 정의
pub trait DoA {
    //구현 내용을 안 넣으면 impl 하는 쪽에서 강제로 구현 해야함
    fn do_b(&self);
    // 구현 내용을 넣으면 이대로 실행 or impl 하는 쪽에서 override 가능
    fn do_c(&self) {
        println!("called do_c");
    }
}


#[test]
fn main() {
    //일부로 trait 을 자료형으로 하는 Vec 정의
    let mut box_vec: Vec<Box<dyn DoA>> = Vec::new();

    let box1 = MyBox { data: "abcc".to_string() };
    let box2 = YourBox { y_data: "abcc".to_string() };
    let box3 = MyBox { data: "abcc".to_string() };
    let box4 = YourBox { y_data: "abcc".to_string() };

    //MyBox 와 YourBox 는 다른 자료형이지만 같은 trait 을 구현했으므로 한곳에 담는게 가능
    box_vec.push(Box::new(box1));
    box_vec.push(Box::new(box2));


    for _box in box_vec {
        _box.do_b();
    }

    hello(box3);
    hello(box4);
}