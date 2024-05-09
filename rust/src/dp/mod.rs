mod adapter;
mod bridge;
mod builder;
mod composite;
mod cqrs_pattern;
mod crp;
mod decorator;
mod di;
mod factory_patterns;
mod mediator;
mod middleware_pattern;
mod new_type;
mod repository;
mod solid;
mod state;
mod strategy;
mod unit_of_work;
mod visitor;

trait Other {
    fn do_other(&self) -> ();
}

trait Some<T: Other> {
    fn something(&self, other: T) -> ();
}

pub struct DoOther {
    unit: (),
}

impl Other for DoOther {
    fn do_other(&self) -> () {
        self.unit.to_owned()
    }
}

pub struct GetSome;

impl<T: Other> Some<T> for GetSome {
    fn something(&self, other: T) -> () {
        let usint = other.do_other();

        assert_eq!(usint, ());

        println!("We can use any other things here: {:?}", usint);

        ()
    }
}

#[test]
fn test_this_unknown_thing() {
    let do_other = DoOther { unit: () };
    let get_some = GetSome {};

    get_some.something(do_other);
}
