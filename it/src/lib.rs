#[cfg(test)]
mod tests {
    use crate::Counter;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_iterator() {
        let new_sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(new_sum, 18);
    }

    #[test]
    fn it_zip_t() {
        let a = [0, 1, 2, 3, 4, 5];
        let b = [0, 1, 2, 3, 4, 5];
        let iter: Vec<_> = a
            .iter()
            .zip(b.iter().skip(1))
            .map(|(x, y)| x * y)
            .filter(|x| x % 3 == 0)
            .collect();
        for i in iter {
            print!("{} ", i);
        }
        println!();
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
