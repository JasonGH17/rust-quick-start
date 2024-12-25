use lib_auth::scheme::argon::ArgonScheme;
use lib_core::{routes, state::AppState};
use tokio::signal;

#[tokio::main]
async fn main() {
    lib_logs::initialize(log::LevelFilter::Trace, lib_logs::Output::Stdout);

    ArgonScheme::init_instance("TEMP_SECRET");

    let app_state = AppState::new().await;

    let app = routes::initialize(app_state.unwrap());

    log::info!(target: "server", "Awaiting connections");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => log::info!("shutting down"),
        _ = terminate => log::info!("shutting down"),
    }
}
