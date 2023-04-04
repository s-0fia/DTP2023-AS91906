use tokio::signal;

// A function that returns a signal for the program to terminate.
pub async fn shutdown_signal() {
    // Get the signal for the windows/MacOS typical termination bind (CTRL+C).
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install the CTRL+C handler!");
    };

    // Get the termination signal for UNIX type systems.
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler for UNIX systems!")
            .recv()
            .await;
    };

    // Implement no handler for other type systems (like IOS, andriod, etc.).
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Select the flag that exists, and return it once activated.
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    // Communicate to the user and exit the program with no error code.
    println!("Shutdown signal received! Starting graceful shutdown...");
    std::process::exit(0);
}