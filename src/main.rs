
// use startup::Application;
// use telemetry::{get_subscriber_with_jeager, init_subscriber};
// use utils::{get_configuration, run_custom_commands};

use rapid_url::commands::run_custom_commands;
use rapid_url::startup::Application;
use rapid_url::telemetry::{get_subscriber_with_jeager, init_subscriber};
use rapid_url::utils::get_configuration;
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        run_custom_commands(args).await?;
    } else {
        let configuration = get_configuration().expect("Failed to read configuration.");
        let subscriber = get_subscriber_with_jeager(
            "rapid-url".into(),
            "info".into(),
            std::io::stdout,
        );
        init_subscriber(subscriber);
        let application = Application::build(configuration).await?;
        application.run_until_stopped().await?;
    }
    Ok(())
}