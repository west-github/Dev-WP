mod foo {
    use crate::{layer::Layer, service::Service};

    pub struct Foo<S> {
        inner: S,
        count: i32,
    }

    impl<S> Foo<S> {
        pub fn new(inner: S, count: i32) -> Self {
            Self { inner, count }
        }
    }

    pub struct FooLayer {
        count: i32,
    }

    impl FooLayer {
        pub fn new(count: i32) -> Self {
            Self { count }
        }
    }

    impl<S> Layer<S> for FooLayer {
        type Service = Foo<S>;

        fn layer(&self, inner: S) -> Self::Service {
            Foo::new(inner, self.count)
        }
    }

    impl<Request, Response, S> Service<Request> for Foo<S>
    where
        Request: std::fmt::Debug + Clone,
        S: Service<Request, Response = Response>,
    {
        type Response = S::Response;
        type Error = S::Error;

        fn call(&mut self, req: Request) -> Result<Self::Response, Self::Error> {
            println!("Service with request: {:?} in count: {}", req, self.count);
            let inner = self.inner.call(req.clone());
            println!("Service with request: {:?} in count: {}", req, self.count);

            inner
        }
    }
}

#[test]
fn test_layer() {
    use super::*;
    use crate::service::Service;
    use foo::*;

    impl Service<String> for () {
        type Response = String;

        type Error = ();

        fn call(&mut self, req: String) -> Result<Self::Response, Self::Error> {
            Ok(req)
        }
    }
    let mut layer = FooLayer::new(1).layer(FooLayer::new(2).layer(FooLayer::new(3).layer(())));
    let res = layer.call(String::from("This is a request"));
}
