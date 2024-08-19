use crate::dto::{Address, Crop, Farm, FarmSize, GrowthEvent, GrowthStage, Location, UserInfo};
use crate::crop_parser::extract;
use crate::seeds::SeedBag;


fn get_location() -> Location {
    let address = Address {
        house_number: 1,
        post_code: "E1 8RU".to_string(),
        street: "Some street".to_string(),
        city: "London".to_string(),
        country: "England".to_string(),
    };

    Location {
        address: address,
        is_virtual: false,
        longitude: None,
        latitude: None,
    }
}

fn get_user_info() -> UserInfo {
    UserInfo {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "Xq9u7@example.com".to_string(),
        phone: None,
        address: None,
        website_url: None,
    }
}

pub fn setup_farm() -> Result<Farm, String> {
    let crops = load_crops_from_bags();
    let location = get_location();
    let size = FarmSize {
        width: 10,
        length: 10,
    };
    let owner = get_user_info();
    Ok(Farm {
        crops,
        location: location,
        size: size,
        owner: owner,
        security_code: "Zsx12-00-RSA".to_string(),
        is_active: true,
        is_trackable: None,
        is_plant_ready: None,
        is_ready_for_harvest: Some(false),
    })
}

/// Load crops from seed bags
/// We tear the bags and return a Vec of crops from the bags. Each bag will contain plantable seeds/crops
/// Args: None
/// Returns: Vec<Crop>
fn load_crops_from_bags() -> Vec<Crop> {
    let seed_bags = SeedBag::tear_bags();
    let mut crops: Vec<Crop> = Vec::new();
    for bag in seed_bags {
        let mut crop = Crop::new(bag.seed_type.get_botanica_name(), bag.seed_type.get_verbose_name(), bag.species, bag.description);
        crop.is_gmo = bag.is_gmo.unwrap();
        for _ in 0..bag.quantity {
            crops.push(crop.clone());
        }
    }
    crops
}