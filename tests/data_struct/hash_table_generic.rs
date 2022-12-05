use std::collections::LinkedList;
use std::fmt::Debug;

#[test]
fn main() {
    let mut hh =  HashTable::new(5);
    hh.put("p1".into(), Person::new("kang", 38));
    hh.put("p2".into(), Person::new("kang2", 38));

    hh.status();

    assert_eq!(hh.get("p2".into()), Some(Person::new("kang2", 38)));
}

#[derive(Debug,Clone)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Person{
        Person{ name: name.to_string(), age }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn age(&self) -> u8 {
        self.age
    }
}
impl PartialEq for Person{

    fn eq(&self, other: &Self) -> bool {
        self.name.eq(other.name()) && self.age.eq(&other.age())
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}


#[derive(Debug)]
struct Node<T>{
    key: String,
    value: T,
}

impl<T> Node<T> {
    pub fn new(key: String, value: T) -> Self{
        Node{
            key,
            value
        }
    }
}

pub struct HashTable<T>{
    datas: Vec<LinkedList<Node<T>>>,
    size: u8,
}

impl<T> HashTable<T>
    where T: Clone + Debug
{
    pub fn new(size:u8)->Self{
        HashTable{
            datas: (0..size).into_iter().map(|_n| LinkedList::new()).collect::<Vec<LinkedList<Node<T>>>>(),
            size,
        }
    }

    pub fn put(&mut self, key:String, value:T) {
        let index: usize = self.find_index(HashTable::<T>::make_hash(&key));
        if let Some(v) = self.datas.get_mut(index){
            let node = Node::new(key, value.clone());
            if let Some(n) = v.iter_mut().find(|n| n.key.eq(&node.key)){
                n.value = value;
            }else {
                v.push_back(node);
            }
        }
    }

    pub fn get(&self, key:String) -> Option<T> {
        let index: usize = self.find_index(HashTable::<T>::make_hash(&key));
        if let Some(v) = self.datas.get(index){
            if let Some(node) = v.iter().find(|n| n.key.eq(&key)){
                return Some(node.value.clone());
            }
        }
        None
    }

    pub fn status(&self){
        for data in &self.datas {
            println!("{:?}",data);
        }
    }

    fn make_hash(key:&String) -> usize{
        let bytes = key.as_bytes();
        bytes.iter().map(|&s| s as usize).sum()
    }

    fn find_index(&self, hash: usize) -> usize{
        hash % self.size as usize
    }
}