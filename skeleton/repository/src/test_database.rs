use std::sync::OnceLock;
use testcontainers::ReuseDirective;
use testcontainers_modules::postgres;
use testcontainers_modules::testcontainers::{Container, ImageExt, runners::SyncRunner};

static POSTGRES: OnceLock<Container<postgres::Postgres>> = OnceLock::new();

// keep this in sync with docker-compose.yml
const TAG: &str = "17-alpine";
const DB_NAME: &str = "iconoclast";
const USER: &str = "iconoclast";
const PASSWORD: &str = "the-final-resistance";

/// if no DATABASE_URL is set,
/// run (reused) postgres testcontainer and set DATABASE_URL for [`sqlx::test`] to pick it up
///
/// "test-harness" workaround for [setup](https://github.com/rust-lang/rust/issues/117668) function
#[ctor::ctor]
fn run_postgres() {
    if std::env::var("DATABASE_URL").is_ok() {
        return;
    }

    let postgres = POSTGRES.get_or_init(|| {
        postgres::Postgres::default()
            .with_db_name(DB_NAME)
            .with_user(USER)
            .with_password(PASSWORD)
            .with_tag(TAG)
            .with_reuse(ReuseDirective::Always)
            .start()
            .unwrap()
    });

    let host_ip = postgres.get_host().unwrap();
    let host_port = postgres.get_host_port_ipv4(5432).unwrap();

    let url = format!("postgres://{USER}:{PASSWORD}@{host_ip}:{host_port}/{DB_NAME}");

    unsafe {
        // set the environment-variable DATABASE_URL. This runs at initialization, so it should not run while other threads try to read DATABASE_URL
        std::env::set_var("DATABASE_URL", url.as_str());
    }
}
