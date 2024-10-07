use once_cell::sync::OnceCell;
use penumbra_proto::view::v1::view_service_server::ViewServiceServer;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use warp::Filter;

mod logging;
mod view_server;

static RUNTIME: OnceCell<Arc<Mutex<Option<Runtime>>>> = OnceCell::new();

#[no_mangle]
pub extern "C" fn start_server() -> bool {
    logging::ensure_subscriber();

    tracing::error!("This is an error message");
    tracing::warn!("This is a warning message");
    tracing::info!("This is an info message");
    tracing::debug!("This is a debug message");
    println!("Hello!!!!!");

    let runtime = RUNTIME.get_or_init(|| Arc::new(Mutex::new(None)));
    let mut runtime_guard = runtime.lock().unwrap();

    if runtime_guard.is_some() {
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
        let vs = view_server::start_test_view_server().await?;
        let vs_tonic = ViewServiceServer::new(vs);

        let server = tonic::transport::Server::builder()
            .accept_http1(true)
            .add_service(tonic_web::enable(vs_tonic))
            .serve("127.0.0.1:3333".parse().unwrap());

        server.await?;

        Ok::<(), anyhow::Error>(())
    });

    runtime.spawn(async {
        let routes = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    });

    *runtime_guard = Some(runtime);
    println!("Server started successfully");
    true
}

#[no_mangle]
pub extern "C" fn stop_server() -> bool {
    if let Some(server) = RUNTIME.get() {
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
