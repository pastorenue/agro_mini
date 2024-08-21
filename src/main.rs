mod crop_parser;
mod dto;
mod plant_service;
mod setup;
mod seeds;
mod weather_service;

use setup::setup_farm;
use crate::plant_service::PlantService;


fn main() {
    // group_crops();
    // split_a_crop();
    let farm = setup_farm().unwrap_or_else(|err| panic!("Error: {}", err));

    let planter = PlantService::new(farm);
    planter.prepare_farm()
        .run();
}
