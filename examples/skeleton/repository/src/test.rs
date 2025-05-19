use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use testcontainers::{runners::AsyncRunner, ContainerAsync, ImageExt};
use testcontainers_modules::postgres;
use tokio::sync::OnceCell;

// keep this in sync with docker-compose.yml
const TAG: &str = "17-alpine";
const DB_NAME: &str = "iconoclast";
const USER: &str = "iconoclast";
const PASSWORD: &str = "the-final-resistance";

static POSTGRES: OnceCell<ContainerAsync<postgres::Postgres>> = OnceCell::const_new();

static POSTGRES_POOL: OnceCell<Pool<sqlx::Postgres>> = OnceCell::const_new();

pub async fn postgres_container() -> &'static ContainerAsync<postgres::Postgres> {
    POSTGRES
        .get_or_init(|| async {
            postgres::Postgres::default()
                .with_db_name(DB_NAME)
                .with_user(USER)
                .with_password(PASSWORD)
                .with_tag(TAG)
                .start()
                .await
                .unwrap()
        })
        .await
}

pub async fn test_pool() -> &'static Pool<sqlx::Postgres> {
    POSTGRES_POOL
        .get_or_init(|| async {
            let container = postgres_container().await;
            let host_ip = container.get_host().await.unwrap();
            let host_port = container.get_host_port_ipv4(5432).await.unwrap();

            let url = format!("postgres://{USER}:{PASSWORD}@{host_ip}:{host_port}/{DB_NAME}");
            let pool = PgPoolOptions::new().connect(&url).await.unwrap();

            sqlx::migrate!("../migrations").run(&pool).await.unwrap();

            pool
        })
        .await
}
