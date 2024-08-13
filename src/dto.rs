use serde::{Deserialize, Serialize};
use serde::de::{self, Deserializer};
use chrono::prelude::*;


fn parse_date<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where 
    D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            return Ok(None);
        }
        DateTime::parse_from_str(&s, "%Y-%m-%d")
            .map(|date| Some(date.with_timezone(&Utc)))
            .map_err(de::Error::custom)
}


#[derive(Deserialize, Debug)]
pub struct Crop {
    pub botanica_name: String,
    pub verbose_name: String,
    pub species: String,
    pub description: Option<String>,
    pub min_bags: u32,
    pub is_harvestable: bool,
    pub is_sowable: bool,
    pub is_gmo: bool,
    #[serde(deserialize_with = "parse_date")]
    pub harvest_date: Option<DateTime<Utc>>,
}

struct Location {
    address: String,
    longitude: Option<f32>,
    latitude : Option<f32>,
    is_virtual: bool,
}

struct Address {
    house_number: u32,
    post_code: u32,
    street: String,
    city: String,
    country: String,
}

struct FarmSize {
    width: u32,
    length: u32
}

struct UserInfo {
    first_name: String,
    last_name: String,
    email: String,
    phone: Option<String>,
    address: Option<Address>,
    website_url: Option<String>,
}

struct Farm {
    crops: Vec<Crop>,
    location: Location,
    size: FarmSize,
    owner: UserInfo,
    security_code: String,
    is_active: bool,
    is_trackable: bool,

}