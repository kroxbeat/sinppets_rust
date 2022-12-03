




#[test]
fn main() {
    let mut num_vec = vec!["a", "b", "c"];

    num_vec.iter().map(|mut a| "ss").collect::<Vec<&str>>();
    println!("{num_vec:?}");
    num_vec.iter_mut().for_each(|mut a| *a = "zz");
    println!("{num_vec:?}");


}

