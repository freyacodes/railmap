use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StatusesPage {
    pub data: Vec<Status>,
    pub links: Links
}

#[derive(Deserialize, Debug)]
pub struct Status {
    pub id: u32,
    pub visibility: u8,
    pub train: Train
}

/// This is a misnomer, it also covers other modes of transport
#[derive(Deserialize, Debug)]
pub struct Train {
    pub category: String,
}

#[derive(Deserialize, Debug)]
pub struct Links {
    pub next: Option<String>
}