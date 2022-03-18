use std::collections::HashMap;

fn main() {
    unsafe {
        let mut hash: HashMap<String, Vec<i32>> = HashMap::new();
        hash.insert(
            "name".to_string(),
            vec![1, 2, 3, 4]
        );
        let h = hash.get_mut("name").unwrap() as *mut Vec<i32>;
        println!("{:?}", h);
        hash.insert(
            "some".to_string(),
            vec![1, 2, 3, 4, 5, 6]
        );
        let h = hash.get_mut("name").unwrap() as *mut Vec<i32>;
        println!("{:?}", h);
        hash.insert(
            "another".to_string(),
            vec![1, 2, 3, 4, 5]
        );
        let h = hash.get_mut("name").unwrap() as *mut Vec<i32>;
        println!("{:?}", h);
    }
}