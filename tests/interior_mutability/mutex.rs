use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn main() {
    let m_str= Arc::new(Mutex::new(String::from("aaa")));
    let mut handels = vec![];

    for i in 0..20 {
        let m_str_c = Arc::clone(&m_str);
        let handle = thread::spawn(move || {
            let mut guard = m_str_c.lock().unwrap();
            guard.push_str(format!("{} ",i).as_str());
        });
        handels.push(handle);
    }

    for handel in handels {
        handel.join().unwrap();
    }
    println!("{:?}",m_str);
}