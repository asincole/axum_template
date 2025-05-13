use anyhow::Context;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct DbConfig {
    pub pool: Pool<Postgres>,
}

impl DbConfig {
    pub async fn new(database_url: &str) -> anyhow::Result<Self> {
        // We create a single connection pool for SQLx that's shared across the whole application.
        // This saves us from opening a new connection for every API call, which is wasteful.
        let db = PgPoolOptions::new()
            // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
            // Since we're using the default superuser, we don't have to worry about this too much,
            // although we should leave some connections available for manual access.
            //
            // If you're deploying your application with multiple replicas, then the total
            // across all replicas should not exceed the Postgres connection limit.
            .max_connections(50)
            .connect(database_url)
            .await
            .context("could not connect to database_url")?;

        // This embeds database migrations in the application binary so we can ensure the database
        // is migrated correctly on startup
        sqlx::migrate!()
            .run(&db)
            .await
            .context("could not run database migrations")?;
        Ok(Self { pool: db })
    }
}

// #[async_trait]
// pub trait DbConfigTrait: Send + Sync {
//     async fn new(&self,database_url: &str) -> anyhow::Result<Self>
//     where
//         Self: Sized;
// }
//
// // Actual implementation in your module
// #[derive(Clone)]
// pub struct DbConfig {
//     pub pool: Pool<Postgres>,
// }
//
// #[async_trait]
// impl DbConfigTrait for DbConfig {
//     async fn new(&self, database_url: &str) -> anyhow::Result<Self> {
//         let db = PgPoolOptions::new()
//             .max_connections(50)
//             .connect(database_url)
//             .await
//             .context("could not connect to database_url")?;
//
//         sqlx::migrate!()
//             .run(&db)
//             .await
//             .context("could not run database migrations")?;
//         Ok(Self { pool: db })
//     }
// }
