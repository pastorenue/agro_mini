#![allow(dead_code)]

use std::{collections::HashMap, fmt::Debug};
use rand::prelude::*;
use serde::Deserialize;

use crate::seeds::SeedType;


#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Crop {
    pub botanica_name: String,
    pub verbose_name: String,
    pub species: String,
    pub is_harvestable: bool,
    pub is_sown: bool,
    pub is_gmo: bool,
    pub description: Option<String>,
    pub harvest_date: Option<String>,
    pub date_rot_detected: Option<String>,
    pub split_size: Option<f32>,
    pub days_in_stage: Option<u32>,
    #[serde(flatten)]
    pub current_stage: Option<GrowthStage>,
}


#[derive(Debug)]
pub struct Location {
    pub address: Address,
    pub is_virtual: bool,
    pub longitude: Option<f32>,
    pub latitude : Option<f32>,
}

#[derive(Debug)]
pub struct Address {
    pub house_number: u32,
    pub post_code: String,
    pub street: String,
    pub city: String,
    pub country: String,
}

#[derive(Debug)]
pub struct FarmSize {
    pub width: u32,
    pub length: u32
}


#[derive(Debug)]
pub struct UserInfo {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<Address>,
    pub website_url: Option<String>,
}


#[derive(Debug)]
pub struct Farm {
    pub crops: Vec<Crop>,
    pub location: Location,
    pub size: FarmSize,
    pub owner: UserInfo,
    pub security_code: String,
    pub is_active: bool,
    pub is_trackable: Option<bool>,
    pub is_plant_ready: Option<bool>,
    pub is_ready_for_harvest: Option<bool>,
}

impl Crop {
    pub fn new(bot_name: String, verbose_name: String, species: String, description: Option<String>) -> Self {
        Self {
            botanica_name: bot_name,
            verbose_name: verbose_name,
            species: species,
            description: description,
            is_harvestable: true,
            is_sown: false,
            is_gmo: false,
            harvest_date: None,
            date_rot_detected: None,
            split_size: Some(1.0),
            days_in_stage: Some(0),
            current_stage: Some(GrowthStage::Seed),
        }
    }

    pub fn simulate_growth(&mut self) -> () {
        let mut rng = thread_rng();
        let rand_days = rng.gen_range(1..5); // Generate a random number between 1 and 5

        println!("Random number: {}", &rand_days);
        self.grow(rand_days as u32);
    }

    pub fn grow(&mut self, days: u32) -> () {
        match self.days_in_stage {
            Some(val) => {
                self.days_in_stage = Some(val + days);
            }
            _ => (),
        }
    }

    pub fn advance_to_next_stage(&mut self) -> () {
        let seed_type = SeedType::from_str(&self.verbose_name).unwrap();
        let max_growth_days = self.current_stage.as_ref().unwrap().get_days(seed_type);
        let current_stage = self.current_stage.as_ref().unwrap();
        if self.days_in_stage.unwrap() == max_growth_days && !self.is_inactive() {
            let next_stage = GrowthStage::next(&current_stage);
            println!("Transiting from {:?} -> {:?}",current_stage, next_stage);
            self.current_stage = Some(next_stage);
        }
    }

    pub fn is_inactive(&self) -> bool {
        self.current_stage == Some(GrowthStage::Failed) || self.current_stage == Some(GrowthStage::Harvest)
    }

    pub fn is_harvested(&self) -> bool {
        self.is_harvestable && self.harvest_date.is_some()
    }

    pub fn split(instance: &mut Crop, size: u32) -> Vec<Crop> {
        let mut splits = Vec::new();
        let new_size = Box::new(instance.split_size.unwrap_or(1.0) / size as f32);
        for _ in 0..size {
            let mut _clone = instance.clone();
            _clone.split_size = Some((*new_size * 100.0).round() / 100.0);
            splits.push(_clone);
        }
        
        instance.split_size = Some((instance.split_size.unwrap_or(*new_size) * 100.0).round() / 100.0); 
        splits
    }

    pub fn sow(&mut self) -> () {
        self.is_sown = true;
    }

    pub fn has_issues(&self) -> bool {
        self.date_rot_detected.is_some() && self.current_stage == Some(GrowthStage::Failed)
    }
}


#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum GrowthStage {
    Seed,
    Seedling,
    Germination,
    Vegetative,
    Flowering,
    Fruiting,
    Maturity,
    Harvest,
    Failed,
}

impl Default for GrowthStage {
    fn default() -> Self {
        Self::Seed
    }
}

#[derive(Debug)]
pub enum GrowthEvent {
    Sync,
    Fail,
}

impl GrowthStage {
    fn get_stage(&self) -> String {
        match self {
            GrowthStage::Seed => String::from("seed"),
            GrowthStage::Germination => String::from("germination"),
            GrowthStage::Seedling => String::from("seedling"),
            GrowthStage::Vegetative => String::from("vegetative"),
            GrowthStage::Flowering => String::from("flowering"),
            GrowthStage::Fruiting => String::from("fruiting"),
            GrowthStage::Maturity => String::from("maturity"),
            GrowthStage::Harvest => String::from("harbest"),
            GrowthStage::Failed => String::from("Failed"),
        }
    }

    pub fn get_days(&self, seed_type: SeedType) -> u32 {
        match self {
            GrowthStage::Seed => match seed_type {
                SeedType::Sunflower(..) => 3,
                SeedType::Pea(..) => 3,
                SeedType::Carrot(..) => 3,
                SeedType::Tomato(..) => 3,
                SeedType::Broccoli(..) => 3,
            },
            GrowthStage::Germination => match seed_type {
                SeedType::Sunflower(..) => 7,
                SeedType::Pea(..) => 7,
                SeedType::Carrot(..) => 10,
                SeedType::Tomato(..) => 7,
                SeedType::Broccoli(..) => 5,  
            },
            GrowthStage::Seedling => match seed_type {
                SeedType::Sunflower(..) => 10,
                SeedType::Pea(..) => 14,
                SeedType::Carrot(..) => 17,
                SeedType::Tomato(..) => 10,
                SeedType::Broccoli(..) => 12,
            },
            GrowthStage::Vegetative => match seed_type {
                SeedType::Sunflower(..) => 12,
                SeedType::Pea(..) => 13,
                SeedType::Carrot(..) => 14,
                SeedType::Tomato(..) => 11,
                SeedType::Broccoli(..) => 21,
            },
            GrowthStage::Flowering => match seed_type {
                SeedType::Sunflower(..) => 8,
                SeedType::Pea(..) => 11,
                SeedType::Carrot(..) => 20,
                SeedType::Tomato(..) => 7,
                SeedType::Broccoli(..) => 16,
            },
            GrowthStage::Fruiting => match seed_type {
                SeedType::Sunflower(..) => 10,
                SeedType::Pea(..) => 7,
                SeedType::Carrot(..) => 9,
                SeedType::Tomato(..) => 5,
                SeedType::Broccoli(..) => 7,
            },
            GrowthStage::Maturity => match seed_type {
                SeedType::Sunflower(..) => 5,
                SeedType::Pea(..) => 5,
                SeedType::Carrot(..) => 5,
                SeedType::Tomato(..) => 5,
                SeedType::Broccoli(..) => 5,
            },
            GrowthStage::Harvest => match seed_type {
                SeedType::Sunflower(..) => 3,
                SeedType::Pea(..) => 3,
                SeedType::Carrot(..) => 5,
                SeedType::Tomato(..) => 1,
                SeedType::Broccoli(..) => 3,
            },
            GrowthStage::Failed => 0,
        }
    }

    pub fn next(instance: &GrowthStage) -> GrowthStage {
        let mut rng = rand::thread_rng();
        let event_idx = rng.gen_range(0..=50);
        let rot_idx =  event_idx % 21 == 0;
        let event = if rot_idx {
            GrowthEvent::Fail
        } else {
            GrowthEvent::Sync
        };
        println!("{:?} -> {:?}", instance, event);
        match instance {
            GrowthStage::Seed => match event {
                GrowthEvent::Sync => GrowthStage::Germination,
                GrowthEvent::Fail => GrowthStage::Failed
            }
            GrowthStage::Germination => match event {
                GrowthEvent::Sync => GrowthStage::Seedling,
                GrowthEvent::Fail => GrowthStage::Failed
            },
            GrowthStage::Seedling => match event {
                GrowthEvent::Sync => GrowthStage::Vegetative,
                GrowthEvent::Fail => GrowthStage::Failed
            },
            GrowthStage::Vegetative => match event {
                GrowthEvent::Sync => GrowthStage::Flowering,
                GrowthEvent::Fail => GrowthStage::Failed
            },
            GrowthStage::Flowering => match event {
                GrowthEvent::Sync => GrowthStage::Fruiting,
                GrowthEvent::Fail => GrowthStage::Failed
            },
            GrowthStage::Fruiting => match event {
                GrowthEvent::Sync => GrowthStage::Maturity,
                GrowthEvent::Fail => GrowthStage::Failed
            },
            GrowthStage::Maturity => match event {
                GrowthEvent::Sync => GrowthStage::Harvest,
                GrowthEvent::Fail => GrowthStage::Failed
            },
            _ => GrowthStage::Failed
        }
    }
}

impl FarmSize {
    fn new(width: u32, length: u32) -> FarmSize {
        FarmSize {
            width,
            length
        }
    }
}

#[derive(Debug)]
struct Stats {
    growth_cycle: u32,
    num_seeds_planted: u32,
    num_harvested: u32,
    num_rotten: u32,
    harvest_date: Option<String>,
}

#[derive(Debug)]
struct HarvestData {
    farm: Farm,
    harvest_stats: HashMap<String, Stats>,
    metadata: Option<HashMap<String, String>>
}
