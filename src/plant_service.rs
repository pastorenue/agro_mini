use rand::prelude::*;
use crate::dto::{Crop, Farm, GrowthStage};
use crate::seeds::SeedType;
use std::time::Duration;
use std::thread;
use std::sync::Mutex;

pub struct PlantService {
    farm: Farm,
    is_all_harvested: bool,
}

static WEEDING_FARM_FREQUENCY: u32 = 7; // every 7 days
static IRRIGATION_FREQUENCY: u32 = 2; // every 2 days
const FERTILIZING_FREQUENCY: u32 = 14; // every 14 days
const DAYS_TO_WAIT_BEFORE_PLANTING: u32 = 7; // 7 days
const FUMIGATION_TIME: u32 = 14; // 14 days after planting
static TOTAL_HARVESTED_SEEDS : Mutex<u32> = Mutex::new(0);

impl PlantService {
    pub fn new(farm: Farm) -> Self {
        Self {
            farm,
            is_all_harvested: false,
        }
    }

    pub fn run(mut self) {
        let mut days_count = 1;

        loop {
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
            self._crop_process();
            days_count += 1;
            
            if *TOTAL_HARVESTED_SEEDS.lock().unwrap() == self.farm.crops.len() as u32 {
                println!("All done!");
                self.is_all_harvested = true;
                break;
            }
        }
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

    fn planting(&self) {
        println!("We will be planting a total of {} seeds today", self.farm.crops.len());

        // Simulate planting
        let mut rng = thread_rng();
        let rand_labour = rng.gen_range(0..10); // Generate a random number between 1 and 5
        println!("Planting started...");
        thread::sleep(Duration::from_secs(rand_labour as u64));

        println!("Successfully planted all seeds!!");
        println!();
    }

    fn irrigate(&self) {
        // Simulate Irrigation
        println!("Irrigation started");
        thread::sleep(Duration::from_secs(2));
        println!("Irrigation completed");
        println!();
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
        println!("Its time to harvest...");
        let num = TOTAL_HARVESTED_SEEDS.lock().unwrap();
        if self.farm.crops.len() == *num as usize {
            self.farm.is_ready_for_harvest = Some(true);
            self.is_all_harvested = true;
        }
        thread::sleep(Duration::from_secs(2));
        println!("Harvesting completed!!!");
        println!();
    }

    fn _crop_process(&mut self) {
        for crop in self.farm.crops.iter_mut() {
            if crop.is_harvestable {
                *crop = crop.clone().grow(1);

                match crop.clone().current_stage.unwrap() {
                    GrowthStage::Germination => {
                        PlantService::_show_update(crop);
                        *crop = Crop::split(crop, 4)[0].clone();
                    },
                    GrowthStage::Seedling => {
                        PlantService::_show_update(crop);
                    }
                    GrowthStage::Flowering => {
                        PlantService::_show_update(crop);
                    },
                    GrowthStage::Fruiting => {
                        PlantService::_show_update(crop);
                    },
                    GrowthStage::Maturity => {
                        PlantService::_show_update(crop);
                    },
                    GrowthStage::Harvest => {
                        PlantService::_show_update(crop);
                        let mut num = TOTAL_HARVESTED_SEEDS.lock().unwrap();
                        *num += 1;
                        // println!("Crop: {:?} -> has been harvested", crop.clone().verbose_name);
                    },
                    _ => {}
                }
            };
        }
    }

    fn _show_update(crop: &mut Crop) {
        let seed_type = &crop.clone().verbose_name;
        let stage = crop.current_stage.as_ref().unwrap();
        let stage_time = stage.get_days(SeedType::from_str(seed_type).unwrap());
        println!("{} -- {}", crop.days_in_stage.unwrap(), stage_time);
        if crop.days_in_stage.unwrap() >= stage_time {
            println!("Crop: {:?} -> {:?}", crop.clone().verbose_name, stage);
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
