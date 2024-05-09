trait UserService {
    fn create_user(&self) -> String;
}

fn service_router(service: impl UserService) {
    service.create_user();

    // Do other stuff here
}

#[test]
fn test_service() {
    struct Payload {
        name: String, // Snip
    }

    impl Payload {
        fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl UserService for Payload {
        fn create_user(&self) -> String {
            todo!()
        }
    }

    service_router(Payload::new(String::from("Some Payload")));
}
