#![allow(dead_code, non_snake_case, unused_variables, unused_doc_comments)]

mod atomic;
mod bytes;
mod copy;
mod cs;
mod deref;
mod dp;
mod enum_over_dp;
mod fp;
mod fs;
mod futures;
mod r#macro;
mod www;
pub use std::format as f;

pub(crate) fn Ok() -> anyhow::Result<()> {
    // More room yeah
    println!("\n\n\n\n***********************\n");
    println!("{:>17}", "TEST RESULTS");
    println!("\n***********************\n");

    anyhow::Result::<()>::Ok(())
}

pub enum Bool {
    True,
    False,
}

impl Bool {
    pub fn is_true(&self) -> bool {
        matches!(self, Bool::True)
    }

    pub fn is_false(&self) -> bool {
        !self.is_true()
    }
}

#[cfg(test)]
mod lib {
    use std::{collections::HashSet, num::NonZeroU8};

    use rand::{thread_rng, Rng};

    fn generate_random_string(length: usize) -> String {
        let char_set = "abcdefghijklmnopqrstuvwxyz1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        (0..length)
            .map(|_| char_set.chars().nth(thread_rng().gen_range(0..char_set.len())).unwrap())
            .collect()
    }

    #[test]
    fn test_len_10() {
        let mut set = vec![];

        for _ in 0..100 {
            set.push(generate_random_string(32));
        }
        set.push(String::from("UUID- V4"));

        for _ in 0..100 {
            set.push(uuid::Uuid::new_v4().to_string().replace("-", ""));
        }

        println!("{:#?}", set);
    }

    #[test]
    fn test_len_6() {
        let mut set = HashSet::new();
        for _ in 0..1000 {
            for _ in 0..1000 {
                set.insert(uuid::Uuid::new_v4().to_string().replace("-", ""));
            }
        }

        assert_eq!(set.len(), 1000_000);
    }

    #[test]
    fn test_uuid() {
        println!("{}", uuid::Uuid::new_v4());
    }

    #[test]
    fn test_char_indices() {
        let char = "SomeString";

        for (index, char) in char.char_indices() {
            println!("{} - {}", index, char);
        }
    }
    #[test]
    fn test_bool() {
        let data = crate::Bool::True;

        if data.is_true() {
            println!("Data didn't match please check the data!")
        }

        assert_eq!(true, data.is_true());
        assert_eq!(false, data.is_false());
    }

    #[test]
    fn same_size() {
        let value = 2 as u8;

        let non_zero = NonZeroU8::new(2);

        assert_eq!(1, std::mem::size_of_val(&value));
        assert_eq!(1, std::mem::size_of_val(&non_zero));
    }

    fn return_func() -> impl FnOnce() -> String {
        let data = String::from("Rust is so amazing");

        || data
    }

    fn return_func_with_moved_owned_type() -> impl Fn() {
        let data = String::from("Rust is so amazing in all ways");

        // Hacker Darker Theme
        // Dobri C07 - A05
        // Dark SynthWave 84
        // Coder Dark Theme

        move || println!("{}", &data)
    }

    fn take_in_func(func: impl FnOnce() -> String) {
        println!("{}", func());
    }

    #[test]
    fn func_tests() {
        let func = return_func_with_moved_owned_type();

        func();

        take_in_func(return_func());
    }

    #[test]
    fn vec_tests() {
        let chain = vec![1, 2, 3, 4];

        chain.iter().skip(2).for_each(|n| println!("{}", n));
    }
}
