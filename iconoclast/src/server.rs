use tokio::io;
use tokio::net::TcpListener;

#[cfg(not(feature = "utoipa"))]
type Router = axum::Router;

#[cfg(feature = "utoipa")]
type Router = utoipa_axum::router::OpenApiRouter;

pub struct Server {
    port: u16,
    router: Router,

    #[cfg(feature = "utoipa")]
    swagger_path: String,

    #[cfg(feature = "utoipa")]
    apispec_path: String,
}

impl Server {
    #[must_use] 
    pub fn new(router: Router) -> Self {
        Self {
            port: 8080,
            router,

            #[cfg(feature = "utoipa")]
            swagger_path: "/swagger-ui".to_string(),

            #[cfg(feature = "utoipa")]
            apispec_path: "/apidoc/openapi.json".to_string(),
        }
    }

    #[must_use] 
    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    #[cfg(feature = "utoipa")]
    pub fn with_swagger_path<S: Into<String>>(mut self, path: S) -> Self {
        self.swagger_path = path.into();
        self
    }

    #[cfg(feature = "utoipa")]
    pub fn with_apispec_path<S: Into<String>>(mut self, path: S) -> Self {
        self.apispec_path = path.into();
        self
    }

    #[cfg(not(feature = "utoipa"))]
    pub async fn start(self) -> Result<(), io::Error> {
        start(self.port, self.router).await
    }

    #[cfg(feature = "utoipa")]
    pub async fn start(self) -> Result<(), io::Error> {
        let (router, api) = self.router.split_for_parts();
        let router = router.merge(
            utoipa_swagger_ui::SwaggerUi::new(self.swagger_path).url(self.apispec_path, api),
        );

        start(self.port, router).await
    }
}

/// # Errors
///
/// May fail to bind to the given port.
pub async fn start(port: u16, app: axum::Router) -> Result<(), io::Error> {
    #[cfg(feature = "listenfd")]
    let listener = {
        let mut listenfd = listenfd::ListenFd::from_env();
        match listenfd.take_tcp_listener(0)? {
            Some(listener) => {
                listener.set_nonblocking(true)?;
                TcpListener::from_std(listener)
            }
            None => TcpListener::bind(("0.0.0.0", port)).await,
        }?
    };
    #[cfg(not(feature = "listenfd"))]
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;

    #[cfg(feature = "livereload")]
    let app = app.layer(tower_livereload::LiveReloadLayer::new());

    axum::serve(listener, app).await
}
