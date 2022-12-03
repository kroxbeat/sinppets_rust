use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// 클로져 로 전달받은 연산이 무거울경우 , 결과값을 cache 해 놓고 두번째 부터 계산을 안하는 로직 샘플

struct Cache<T>
    where T: Fn(u32) -> u32
{
    cal: T,
    value: HashMap<u32,u32>,
}

impl<T> Cache<T>
    where T: Fn(u32) -> u32
{
    fn new(cal: T) -> Cache<T> {
        Cache { cal, value: HashMap::new() }
    }
    fn get_value(&mut self, arg: u32) -> u32{
        if let Some(v) = self.value.get(&arg){
            return *v;
        }else {
            let v = (self.cal)(arg);
            self.value.insert(v, v);
            return v;
        }
    }
}

#[test]
fn main() {
    generate_workout();
}


pub fn generate_workout() {

    let mut expensive_result = Cache::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });
    for ii in 0..10 {
        println!(
            "Today, do {} pushups!",
            expensive_result.get_value(ii)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.get_value(ii)
        );
    }
    for ii in 0..10 {
        println!(
            "Today, do {} pushups!",
            expensive_result.get_value(ii)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.get_value(ii)
        );
    }

}