#[allow(unused_macros)]
macro_rules! timed_data_lock {
    ($data:ident, $name:literal) => {
        {
            use std::time::Instant;
            let now = Instant::now();
            let unlocked = $data.lock().unwrap();
            println!("Timed lock '{}' ns:\t{}", $name, now.elapsed().as_nanos());
            unlocked
        }
    };
}