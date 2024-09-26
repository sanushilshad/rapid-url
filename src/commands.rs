
use secrecy::ExposeSecret;
use sqlx::PgPool;

use crate::utils::{configure_database, generate_jwt_token_for_user, get_configuration, get_user_id};

#[tracing::instrument(name = "Default Migration")]
pub async fn run_migrations() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    configure_database(&configuration.database).await;
}





#[tracing::instrument(name = "Generate user token")]
pub async fn generate_user_token(username: &str) {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_with(configuration.database.with_db()).await.expect("Failed to connect to Postgres.");
    let token = match get_user_id(&connection_pool, username).await {
        Ok(Some(id)) =>  
            generate_jwt_token_for_user(id, configuration.secret.jwt.expiry, &configuration.secret.jwt.secret).map_err(|e| anyhow::anyhow!("JWT generation error: {}", e)),
        Ok(None) => Err(anyhow::anyhow!("User not found")),
        Err(e) => Err(anyhow::anyhow!("Database error: {}", e)),
    };
    eprint!("Token for {} is: {}", username, token.unwrap().expose_secret())
}



#[tracing::instrument(name = "Run custom command")]
pub async fn run_custom_commands(args: Vec<String>) -> Result<(), anyhow::Error> {
    if args.len() > 1 {
        if args[1] == "migrate" {
            run_migrations().await;
        }
        if args[1] == "generate_token" {
            if args.len() > 2 {
                generate_user_token(&args[2]).await;
            }

        }


    } else {
        eprintln!("Invalid command. Use Enter a valid command");
    }

    Ok(())
}
