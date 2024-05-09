mod adapter;
mod json;
mod xml;
use anyhow::Result;

pub trait Save {
    fn save(&self) -> Result<()>;
}

pub fn save_files<T>(args: &T) -> Result<()>
where
    T: Save,
{
    args.save()
}

pub fn save(data: &str) -> Result<()> {
    println!("Saving the data to {}", data);

    Ok(())
}

#[test]
fn test_json() -> Result<()> {
    let json = self::json::Json::new([157, 153, 152, 159]);
    let res = save_files(&self::adapter::Adapter(&json))?;

    assert_eq!(res, ());

    Ok(())
}

#[test]
fn test_xml() -> Result<()> {
    let xml = self::xml::Xml::new(String::from("Yes we got this"));
    let res = save_files(&xml)?;

    assert_eq!(res, ());

    Ok(())
}
