#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use uuid::Uuid;
    use crate::startup::get_connection_pool;
    use crate::utils::{delete_short_urls, generate_short_url, get_configuration, get_original_url, insert_url};

    pub async fn get_test_pool() -> PgPool {
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.application.port = 0;
        get_connection_pool(&configuration.database)
    }

    #[tokio::test]
    async fn test_insert_url() {
        let short_url = generate_short_url();
        let long_url= "google.com";
        let pool = get_test_pool().await;
        let response =  insert_url(&pool, long_url, &short_url, &Uuid::new_v4()).await; 
        assert!(response.is_ok());
        let _ = delete_short_urls(&pool).await;
    }

    #[tokio::test]
    async fn test_fetch_url() {
        let short_url = generate_short_url();
        let long_url= "google.com";
        let pool = get_test_pool().await;
        let _ =  insert_url(&pool, long_url, &short_url, &Uuid::new_v4()).await; 
        let response = get_original_url(&pool, &short_url).await;
        assert!(response.is_ok());
        assert!(response.unwrap().is_some());
        let _ = delete_short_urls(&pool).await;
 
    }
}