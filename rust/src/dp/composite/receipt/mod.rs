mod invoice;
mod paper_receipt;

pub trait Summary {
    fn summarize(&self) -> anyhow::Result<()>;
}

#[cfg(test)]
mod receipt_order {
    use super::{
        invoice::{Invoice, Item},
        paper_receipt::PaperReceipt,
        Summary,
    };

    #[test]
    fn receipt() -> anyhow::Result<()> {
        let receipt = PaperReceipt::new();

        let _receipt = PaperReceipt::new().add(Box::new(Invoice::new(vec![Item::new(
            "Tomato".into(),
            20,
            3,
        )])));

        let receipt = receipt.add(Box::new(_receipt));

        receipt.summarize()?;

        Ok(())
    }
}
