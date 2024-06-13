pub trait Service<Request> {
    type Response;
    type Error;

    fn call(&mut self, req: Request) -> Result<Self::Response, Self::Error>;
}
