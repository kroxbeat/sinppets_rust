use std::rc::Rc; //레퍼런스 카운트 ( 복수 소유자 를 갖게 하는 ) .clone() , Rc::clone(&T) , Rc::strong_count(&T) - 소유자 개수 확인

#[allow(dead_code)]
#[derive(Debug)]
struct City {
    name: Rc<String>,
    population: u32,
    city_history: Rc<String>, // String inside an Rc
}

#[derive(Debug)]
struct CityData {
    names: Vec<Rc<String>>,
    histories: Vec<Rc<String>>, // A Vec of Strings inside Rcs
}

#[test]
fn main() {
    let seoul = City {
        name: Rc::new("seoul".to_string()),
        population: 3200000,
        city_history: Rc::new("seoul is ... ".to_string()),
    };

    let mut cities = CityData { names: vec![], histories: vec![] };

    println!("{seoul:?}");
    //& 참조자를 갖도록 정의를 변경할수 있지만 그러면 라이프타임 을 명시해야하고 , 최소 전체리스트 보다 오래 살아야한다
    cities.names.push(seoul.name.clone());
    cities.histories.push(seoul.city_history.clone());

    println!("{:?}", cities);
}
