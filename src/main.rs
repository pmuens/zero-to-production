use std::fmt::{Debug, Display};
use tokio::task::JoinError;
use zero_to_production::{
    configuration::get_configuration, issue_delivery_worker::run_worker_util_stopped,
    startup::Application, telemetry::get_subscriber, telemetry::init_subscriber,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("zero-to-production".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_util_stopped(configuration));

    tokio::select! {
        o = application_task => report_exit("API", o),
        o = worker_task => report_exit("Background Worker", o)
    };

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited.", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(error.cause_chain = ?e, error.message = %e, "{} failed", task_name)
        }
        Err(e) => {
            tracing::error!(error.cause_chain = ?e, error.message = %e, "{} task failed to complete.", task_name)
        }
    }
}
