#[macro_export]
macro_rules! use_lock {
    ($lock:expr) => {
        $lock.lock()
    };
}

#[macro_export]
macro_rules! clone {
    ($expr:expr) => {
        $expr.clone()
    };
}

#[macro_export]
macro_rules! arc_mutex {
    ($value:expr) => {{
        use std::sync::{Arc, Mutex};
        Arc::new(Mutex::new($value))
    }};
}

#[macro_export]
macro_rules! string {
    () => {
        String::new()
    };

    ($content:expr) => {
        String::from($content)
    };

    ($content:expr, $capacity:expr) => {{
        let mut str = String::with_capacity($capacity);
        str.push_str($content);
        str
    }};
}

#[macro_export]
macro_rules! duration_since {
    ($earlier:expr) => {{
        use std::time::Instant;
        Instant::now().duration_since($earlier)
    }};
}
