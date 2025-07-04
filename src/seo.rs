use serde::Serialize;

#[derive(Serialize)]
pub struct SchemaBrand<'a> {
    #[serde(rename = "@type")]
    pub type_of: &'a str,
    pub name: &'a str,
}

#[derive(Serialize)]
pub struct SchemaOffer<'a> {
    #[serde(rename = "@type")]
    pub type_of: &'a str,
    pub url: String,
    pub price_currency: &'a str,
    pub price: String,
    pub availability: &'a str,
    pub item_condition: &'a str,
}

#[derive(Serialize)]
pub struct SchemaProduct<'a> {
    #[serde(rename = "@context")]
    pub context: &'a str,
    #[serde(rename = "@type")]
    pub type_of: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub sku: String,
    pub image: &'a [String],
    pub brand: SchemaBrand<'a>,
    pub offers: SchemaOffer<'a>,
}
