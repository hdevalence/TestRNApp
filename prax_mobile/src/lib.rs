#[no_mangle]
extern "C" fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

use warp::Filter;

#[no_mangle]
pub extern "C" fn start_server() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

            warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
