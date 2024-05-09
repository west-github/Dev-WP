use super::{file_saver::FileSaverDataCollector, invoice_item::InvoiceItem};
use crate::f;
use anyhow::Result;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Invoice<'a> {
    pub items: Vec<InvoiceItem<'a>>,
}

impl<'a> Invoice<'a> {
    pub fn new<T>(value: T) -> Self
    where
        T: IntoIterator<Item = InvoiceItem<'a>>,
    {
        let items = value.into_iter().collect();

        Self { items }
    }

    pub fn add_item<'b: 'a>(&mut self, item: InvoiceItem<'b>) {
        self.items.push(item);
    }
}

impl<'a> FileSaverDataCollector for Invoice<'a> {
    fn collector(&self) -> Result<String> {
        Ok(f!("{}", self))
    }
}

impl<'a> Display for Invoice<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let total: i32 = self.items.iter().map(|item| item.price).sum();

        let items = self
            .items
            .iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<_>>()
            .join("");

        write!(f, "{}\nInvoice Total: {} Item: {}", items, total, self.items.len())
    }
}
