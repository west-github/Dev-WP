use std::fmt::Debug;

mod old;

trait IHandler<T> {
    fn next(&self, request: &T) -> Option<&dyn IHandler<T>>;

    fn handle(&self, request: &T) {
        self.next(request).map(|next| next.handle(request));
    }
}

struct Monkey<'a, T> {
    name: String,
    error: bool,

    next: Option<&'a dyn IHandler<T>>,
}

impl<'a, T> Monkey<'a, T> {
    fn new(name: String, error: bool, next: Option<&'a dyn IHandler<T>>) -> Self {
        Self { name, error, next }
    }
}

impl<'a, T: Debug> IHandler<T> for Monkey<'a, T> {
    fn next(&self, request: &T) -> Option<&'a dyn IHandler<T>> {
        self.next
    }

    fn handle(&self, request: &T) {
        if self.error {
            println!("{} error early so we cutting the chain", self.name);
            return;
        }

        println!("{} approved, {:?}", self.name, request);

        <Monkey<T> as IHandler<T>>::handle(&self, request)
    }
}

#[test]
fn crp_test() {
    #[derive(Debug)]
    struct Request {
        content: String,
    }

    impl Request {
        fn new(content: String) -> Self {
            Self { content }
        }
    }

    let nut: Monkey<Request> = Monkey::new("Nut".into(), false, None);
    let monkey = Monkey::new("Banana".into(), false, Some(&nut));

    monkey.handle(&Request::new("foods".into()))
}
