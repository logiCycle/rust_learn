use std::collections::HashMap;
struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: std::hash::Hash + std::cmp::Eq + Clone,
    V: Clone,
{
    caculation: T,
    value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: std::hash::Hash + std::cmp::Eq + Clone,
    V: Clone,
{
    fn new(caculation: T) -> Cacher<T, K, V> {
        Cacher {
            caculation,
            value: HashMap::new(),
        }
    }

    fn get_value(&mut self, arg: K) -> &V {
        let map = &mut self.value;
        match map.contains_key(&arg) {
            true => &map[&arg],
            false => {
                let value = (self.caculation)(arg.clone());
                map.insert(arg.clone(), value);
                &map[&arg]
            }
        }
    }

}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {

use crate::Cacher;

    #[test]
    fn it_cacher() {
        let mut cacher = Cacher::new(|x| x);
        let v = cacher.get_value(2);
        print!("{}\n", v);


    }

    // fn it_cacher_str() {
    //     let str = String::from("2313");
    //     let str_2 = str + "1";
    //     let mut cacher_str = Cacher::new(|x| x);

    //     let mut v = cacher_str.get_value(String::from("22"));
    // }
}
