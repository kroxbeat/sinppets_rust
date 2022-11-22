use std::collections::LinkedList;

#[test]
fn main() {
    let mut hh =  HashTable::new(5);
    for i in 0..20 {
        hh.put(format!("kang{}",i), format!("value {}",i));
    }
    hh.status();
    for i in 0..20 {
        hh.put(format!("kang{}",i), format!("value2 {}",i));
    }
    hh.status();
    println!("kang19 value is {}",hh.get("kang19".into()));
}


#[derive(Debug)]
struct Node{
    key: String,
    value: String,
}

impl Node {
    pub fn new(key: String, value: String) -> Self{
        Node{
            key,
            value
        }
    }
}

pub struct HashTable{
    datas: Vec<LinkedList<Node>>,
    size: u8,
}

impl HashTable {
    pub fn new(size:u8)->Self{
        HashTable{
            datas: (0..size).into_iter().map(|_n| LinkedList::new()).collect::<Vec<LinkedList<Node>>>(),
            size,
        }
    }

    pub fn put(&mut self, key:String, value:String){
        let index: usize = self.find_index(HashTable::make_hash(&key));
        if let Some(v) = self.datas.get_mut(index){
            let node = Node::new(key, value.clone());
            if let Some(n) = v.iter_mut().find(|n| n.key.eq(&node.key)){
                n.value = value;
            }else {
                v.push_back(node);
            }
        }
    }

    pub fn get(&self, key:String) -> String{
        let index: usize = self.find_index(HashTable::make_hash(&key));
        if let Some(v) = self.datas.get(index){
            if let Some(node) = v.iter().find(|n| n.key.eq(&key)){
                return node.value.clone();
            }
        }
        "no data".to_string()
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