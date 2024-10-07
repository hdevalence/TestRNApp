use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use warp::Filter;

static SERVER: OnceCell<Arc<Mutex<Option<Runtime>>>> = OnceCell::new();

#[no_mangle]
pub extern "C" fn start_server() -> bool {
    let server = SERVER.get_or_init(|| Arc::new(Mutex::new(None)));
    let mut server_guard = server.lock().unwrap();

    if server_guard.is_some() {
        println!("Server is already running");
        return false;
    }

    let runtime = match Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            println!("Failed to create runtime: {}", e);
            return false;
        }
    };

    runtime.spawn(async {
        let routes = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    });

    *server_guard = Some(runtime);
    println!("Server started successfully");
    true
}

#[no_mangle]
pub extern "C" fn stop_server() -> bool {
    if let Some(server) = SERVER.get() {
        let mut server_guard = server.lock().unwrap();
        if let Some(runtime) = server_guard.take() {
            // Shutdown the runtime without waiting
            runtime.shutdown_background();
            println!("Server stopped successfully");
            return true;
        }
    }
    println!("No server was running");
    false
}

#[no_mangle]
pub extern "C" fn sum(a: i32, b: i32) -> i32 {
    a + b
}
