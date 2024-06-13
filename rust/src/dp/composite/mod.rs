mod private_test;
mod receipt;
use private_test::*;

#[derive(Debug)]
struct Leaf {
    branch: i32,
}

impl Summary for Leaf {
    fn summarize(&mut self) -> anyhow::Result<()> {
        println!("I'm a leaf with branch {}", self.branch);
        Ok(())
    }
}

struct Branch {
    branch: Vec<Box<dyn Summary>>,
}

impl Composite for Branch {
    fn store(&mut self) -> &mut Vec<Box<dyn Summary>> {
        &mut self.branch
    }
}

#[test]
fn test_composite() -> anyhow::Result<()> {
    let mut branch = Branch { branch: vec![] };
    let mut first = 10;

    (0..2).for_each(|_| {
        println!("{}", first);
        let mut _branch = Branch { branch: vec![] };
        for i in (first - 10)..first {
            _branch.add(Leaf { branch: i });
        }
        branch.add(_branch);
        first += 10;
    });

    branch.summarize()?;

    Ok(())
}
