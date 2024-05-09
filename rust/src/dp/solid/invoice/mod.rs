#![allow(dead_code, unused_variables)]

mod file_saver;
mod invoice;
mod invoice_item;

#[cfg(test)]
mod invoice_test {

    use super::{file_saver::FileSaver, invoice::Invoice, invoice_item::InvoiceItem};

    fn build_invoice<'a>() -> Invoice<'a> {
        let mut invoice = Invoice::new(vec![
            InvoiceItem::new("Tomato", 20),
            InvoiceItem::new("Orange", 30),
            InvoiceItem::new("Potato", 50),
        ]);

        invoice.add_item(InvoiceItem::new("Banana", 50));

        invoice
    }

    #[test]
    fn test_invoice() {
        let invoice = build_invoice();

        println!("{}", invoice);

        assert_eq!(4, invoice.items.len());
    }

    #[test]
    fn test_file_saver() -> anyhow::Result<()> {
        let invoice = build_invoice();

        let file_saver = FileSaver::new("./", "file_001");

        let res = file_saver.save(invoice)?;

        // Invoice is anything that's collectible
        // let res = file_saver.save(invoice)?;

        assert_eq!(res, ());

        Ok(())
    }
}
