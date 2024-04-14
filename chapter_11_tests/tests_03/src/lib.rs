use std::{thread, time};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
        // 3,600,000 milliseconds
        let one_hour = time::Duration::from_millis(3600000);
        let now = time::Instant::now();

        thread::sleep(one_hour);

        assert!(now.elapsed() >= one_hour);
    }
}
