use std::collections::HashMap;
use std::thread;
use std::time::Duration;

/*
 * Try modifying `Cacher` to hold a hash map rather than a single value.
 * The keys of the hash map will be the arg values that are passed in,
 * and the values of the hash map will be the result of calling the closure on that key.
 * Instead of looking at whether `self.value` directly as a `Some` or a `None` value,
 * the `value` function will look up the `arg` in the hash map and return the value if it's
 * present. If it's not present, the `Cacher` will call the closure and save the resulting value in
 * the hash map associated with its `arg` value.
 */

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
        return;
    }
    if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes",
            expensive_result.value(intensity)
        );
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number)
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
