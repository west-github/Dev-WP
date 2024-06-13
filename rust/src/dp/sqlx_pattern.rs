use anyhow::Result;
mod all_trait {
    macro_rules! create_trait {
        ($trait:ident, $func_name:ident) => {
            #[doc = "Create trait with the given identifier"]
            pub trait $trait {
                fn $func_name() -> anyhow::Result<()>;
            }
        };
    }

    pub trait C<B, A> {
        fn do_c(args: B) -> A;
    }

    create_trait!(A, do_a);
    create_trait!(B, do_b);

    pub trait All {
        type A: A;

        type B: B;

        type C: C<B, A>;

        fn use_all(&self) -> Result<()>;
    }
}

use all_trait::*;

struct DoA {}

impl A for DoA {
    fn do_a() -> anyhow::Result<()> {
        Ok(())
    }
}

struct DoB {}

impl B for DoB {
    fn do_b() -> anyhow::Result<()> {
        Ok(())
    }
}

struct DoC {}

impl<B, A> C<B, A> for DoC {
    fn do_c(args: B) -> A {
        Ok(())
    }
}

struct DoAll {}

impl All for DoAll {
    type A = DoA;

    type B = DoB;

    type C = DoC;

    fn use_all(&self) -> Result<()> {
        Self::A::do_a();
        Self::B::do_b();
        // Self::C::do_c(args);
        Ok(())
    }
}
