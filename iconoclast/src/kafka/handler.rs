pub trait MessageHandler<P, E>
where
    E: std::error::Error,
{
    fn handle(&self, payload: P) -> impl Future<Output = Result<(), E>>;
}
