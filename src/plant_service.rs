#![allow(dead_code)]

use chrono::Utc;
use rand::prelude::*;
use crate::dto::{Crop, Farm, GrowthStage};
use crate::seeds::SeedType;
use std::time::Duration;
use std::thread;
use std::sync::Mutex;
use std::collections::HashMap;
use crate::weather_service::WeatherCondition;


pub struct PlantService {
    farm: Farm,
    is_all_harvested: bool,
    planting_is_initiated: bool,
    harvest_stats: HashMap<String, HashMap<String, u32>>,
}

static WEEDING_FARM_FREQUENCY: u32 = 7; // every 7 days
static IRRIGATION_FREQUENCY: u32 = 3; // every 2 days
const FERTILIZING_FREQUENCY: u32 = 14; // every 14 days
const DAYS_TO_WAIT_BEFORE_PLANTING: u32 = 7; // 7 days
const FUMIGATION_TIME: u32 = 14; // 14 days after planting
const PLANTING_WINDOW: u32 = 70; // 70 days - from planting to harvest
static TOTAL_HARVESTED_SEEDS : Mutex<u32> = Mutex::new(0);
static TOTAL_ROTTEN_SEEDS: Mutex<u32> = Mutex::new(0);
const CLEAR: &str = "\x1B[2J\x1B[1;1H"; // clear the console

impl PlantService {
    pub fn new(farm: Farm) -> Self {
        Self {
            farm,
            planting_is_initiated: false,
            is_all_harvested: false,
            harvest_stats: HashMap::new(),
        }
    }

    pub fn run(mut self) {
        let mut days_count = 1;
        println!("Running farm simulation for {:?}", self.farm);

        loop {
            println!();
            println!("Day: {}", days_count);
            println!("----------------------");
            if days_count == 1 {self.irrigate()}; // irrigate before planting
            if days_count == DAYS_TO_WAIT_BEFORE_PLANTING {self.planting()}; // plant the seeds 
            if days_count % WEEDING_FARM_FREQUENCY == 0 {self.weed()}; // weed the farm
            if days_count < 40 && days_count % FERTILIZING_FREQUENCY == 0 {
                self.apply_fertilizer() // apply fertilizer
            };
            if days_count % IRRIGATION_FREQUENCY == 0 {self.irrigate()}; // irrigate after planting
            if days_count == FUMIGATION_TIME {self.fumigate_seedlings()}; // fumigate seedlings
            
            // process crop activities
            if self.planting_is_initiated {
                self._crop_process(days_count);
            }
            days_count += 1;
            
            if days_count >= PLANTING_WINDOW { // harvesting can take place any time from now.
                self.harvest();
                self.end_farming_simulation(); // Terminate farming simulation.
                if self.is_all_harvested {
                    break;
                }
            }
        }

        println!("Farm simulation completed!!!");
        println!("Simumation Stats: {:?}", self.harvest_stats);
    }

    fn fumigate_seedlings(&self) {
        println!("Fumigating seedlings");
        thread::sleep(Duration::from_secs(2));
        println!("Seedlings fumigated successfully!!");
        println!();
    }

    pub fn prepare_farm(self) -> Self {
        println!("Tilling the farm");

        let mut rng = thread_rng();
        let mut delay = -1;

        while delay != 0 {
            let rand_labour = rng.gen_range(0..5); // Generate a random number between 1 and 5
            println!("We are still preparing the farm...");
            if rand_labour == 3 {
                println!("This land feels really hard. But we are almost done.");
            }
            thread::sleep(Duration::from_secs(rand_labour as u64));

            delay = rand_labour;
        }

        println!("Farm preparation completed!!!");
        println!();

        self
    }

    fn planting(&mut self) {
        println!("We will be planting a total of {} seeds today", self.farm.crops.len());
        // Simulate planting
        let mut rng = thread_rng();
        let rand_labour = rng.gen_range(0..10); // Generate a random number between 1 and 5
        println!("Planting started...");
        thread::sleep(Duration::from_secs(rand_labour as u64));

        self.planting_is_initiated = true;

        println!("Successfully planted all seeds!!");
        println!();
    }

    fn irrigate(&self) {
        // Simulate Irrigation
        let weather_condition = WeatherCondition::generate_random_weather_condition();
        if !(weather_condition == WeatherCondition::Rainy || weather_condition == WeatherCondition::Stormy) {
            println!("Irrigation started");
            thread::sleep(Duration::from_secs(2));
            println!("Irrigation completed");
            println!();
        } else {
            println!("No need to irrigate. Today's weather is {:?}", weather_condition);
        }
    }

    fn apply_fertilizer(&self) {
        // Simulate Fertilizer application
        println!("Fertilizer application started");
        thread::sleep(Duration::from_secs(2));
        println!("Fertilizer application completed");
        println!();
    }

    fn harvest(&mut self) {
        // Simulate Harvest
        let mut rng = thread_rng();
        let rand_harvest_duration = rng.gen_range(1..5); // Generate a random number between 1 and 5
        thread::sleep(Duration::from_secs(rand_harvest_duration));
        println!();
    }

    fn end_farming_simulation(&mut self) {
        // Simulate Termination
        let num_harvested = TOTAL_HARVESTED_SEEDS.lock().unwrap();
        let num_rotten = TOTAL_ROTTEN_SEEDS.lock().unwrap();
        if self.farm.crops.len() == (*num_rotten + *num_harvested) as usize {
            let mut _stats = Some(HarvestStats {num_harvested: *num_harvested, num_rotten: *num_rotten});
            // self.harvest_stats.insert("rotten".to_string(), *num_rotten);
            // self.harvest_stats.insert("harvested".to_string(), *num_harvested );
            self.farm.is_ready_for_harvest = Some(true);
            self.is_all_harvested = true;
        }
    }

    fn _crop_process(&mut self, current_days: u32) {
        for crop in self.farm.crops.iter_mut() {
            if crop.is_harvestable {
                crop.grow(1);
                match crop.current_stage {
                    Some(GrowthStage::Seed) => {
                        Crop::sow(crop);
                        PlantService::_check_for_update(crop);
                    },
                    Some(GrowthStage::Germination) => {
                        PlantService::_check_for_update(crop);
                    },
                    Some(GrowthStage::Seedling) => {
                        PlantService::_check_for_update(crop);
                    }
                    Some(GrowthStage::Vegetative) => {
                        PlantService::_check_for_update(crop);
                    }
                    Some(GrowthStage::Flowering) => {
                        PlantService::_check_for_update(crop);
                    },
                    Some(GrowthStage::Fruiting) => {
                        PlantService::_check_for_update(crop);
                    },
                    Some(GrowthStage::Maturity) => {
                        PlantService::_check_for_update(crop);
                    },
                    Some(GrowthStage::Harvest) => {
                        let mut num = TOTAL_HARVESTED_SEEDS.lock().unwrap();
                        if !crop.is_harvested() {
                            *num += 1;
                            crop.harvest_date = Some(Utc::now().to_string());
                            let inner_map = self.harvest_stats
                                                                        .entry(crop.verbose_name.to_string())
                                                                        .or_insert_with(HashMap::new);
                            *inner_map.entry("havested".to_string())
                                .or_insert(0) += 1;
                            println!(
                                "Crop: {:?} -> Harvest completed after {} days!!!",
                                crop.verbose_name,
                                current_days
                            );
                        }
                    },
                    Some(GrowthStage::Failed) => {
                        if !crop.has_issues() {
                            let mut num = TOTAL_ROTTEN_SEEDS.lock().unwrap();
                            *num += 1;
                            crop.date_rot_detected = Some(Utc::now().to_string());
                            println!("Crop: {:?} -> Failed after {} days!!!", crop.verbose_name, current_days);
                            let inner_map = self.harvest_stats
                                                                        .entry(crop.verbose_name.to_string())
                                                                        .or_insert_with(HashMap::new);
                            *inner_map.entry("failed".to_string())
                                .or_insert(0) += 1;
                        }
                    }
                    _ => ()
                }
            };
        }
    }

    fn _check_for_update(crop: &mut Crop) -> () {
        let seed_type = crop.verbose_name.as_str();
        let stage = crop.current_stage.as_ref().unwrap();
        let stage_time = stage.get_days(SeedType::from_str(seed_type).unwrap());
        if (crop.days_in_stage.unwrap() == stage_time) && !crop.is_inactive() {
            print!("{:?} --> {:?} stage | ", crop.verbose_name, stage);
            crop.advance_to_next_stage();
            crop.days_in_stage = Some(0);
        }
    }

    fn weed(&self) {
        // Simulate Weeding
        println!("Weeding started");
        thread::sleep(Duration::from_secs(2));
        println!("Weeding completed");
        println!();
    }
}

struct HarvestStats {
    num_harvested: u32,
    num_rotten: u32
}