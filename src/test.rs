use std::collections::HashMap;

fn main() {
    unsafe {
        let mut hash: HashMap<String, Vec<i32>> = HashMap::new();
        let mut vec1 = vec![1, 2, 3, 4];
        hash.insert(
            "name".to_string(),
            vec1
        );
        if let Some(v) = hash.get_mut("name") {
            let mut vec2 = &mut *p as &mut Vec<i32>;
            vec2.push(5);

            println!("{:?}", hash.get("name"));
        }


    }
}