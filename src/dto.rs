struct Crop {
    botanica_name: String,
    verbose_name: String,
    species: String,
    description: Option<String>,
    min_bags: u32,
    is_harvestable: bool,
    is_sowable: bool,
    is_gmo: bool,
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