use std::fmt::Debug;
use rand::prelude::*;
use serde::Deserialize;


#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Crop {
    pub botanica_name: String,
    pub verbose_name: String,
    pub species: String,
    pub description: Option<String>,
    pub min_bags: u32,
    pub is_harvestable: bool,
    pub is_sowable: bool,
    pub is_gmo: bool,
    pub harvest_date: Option<String>,
    pub split_size: Option<f32>,
    pub days_in_stage: Option<u32>,
    #[serde(flatten)]
    pub current_stage: Option<GrowthStage>,
}

struct Location {
    address: Address,
    is_virtual: bool,
    longitude: Option<f32>,
    latitude : Option<f32>,
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

impl Crop {

    fn new() -> Self {
        Self {
            botanica_name: String::new(),
            verbose_name: String::new(),
            species: String::new(),
            description: None,
            min_bags: 0,
            is_harvestable: false,
            is_sowable: false,
            is_gmo: false,
            harvest_date: None,
            split_size: Some(1.0),
            days_in_stage: None,
            current_stage: Some(GrowthStage::Seed),
        }
    }

    pub fn simulate_growth(self) -> Self {
        let mut rng = thread_rng();
        let rand_days = rng.gen_range(1..5); // Generate a random number between 1 and 5

        println!("Random number: {}", &rand_days);
        return self.grow(rand_days as u32);
    }

    pub fn grow(mut self, days: u32) -> Self {
        match self.days_in_stage {
            Some(val) => {
                self.days_in_stage = Some(val + days);
            }
            _ => self.days_in_stage = Some(days),
        }

        self._advance_stage();
        self
    }

    fn _advance_stage(&mut self) -> () {
        let max_growth_days = self.current_stage.as_ref().unwrap().get_days();
        if self.days_in_stage.unwrap() >= max_growth_days {
            self.days_in_stage = Some(0);
            match self.current_stage {
                Some(GrowthStage::Seed) => self.current_stage = Some(GrowthStage::Seedling),
                Some(GrowthStage::Seedling) => self.current_stage = Some(GrowthStage::Germination),
                Some(GrowthStage::Germination) => self.current_stage = Some(GrowthStage::Vegetative),
                Some(GrowthStage::Vegetative) => self.current_stage = Some(GrowthStage::Flowering),
                Some(GrowthStage::Flowering) => self.current_stage = Some(GrowthStage::Fruiting),
                Some(GrowthStage::Fruiting) => self.current_stage = Some(GrowthStage::Maturity),
                Some(GrowthStage::Maturity) => self.current_stage = Some(GrowthStage::Harvest),
                _ => self.current_stage = Some(GrowthStage::Rot)
            };
        }
    }

    fn is_harvested(&self) -> bool {
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
    Rot,
}

impl Default for GrowthStage {
    fn default() -> Self {
        Self::Seedling
    }
}


pub enum GrowthEvent {
    Sync,
    Fail,
}

impl GrowthStage {
    fn get_stage(&self) -> String {
        match self {
            GrowthStage::Seed => String::from("seed"),
            GrowthStage::Seedling => String::from("seedling"),
            GrowthStage::Germination => String::from("germination"),
            GrowthStage::Vegetative => String::from("vegetative"),
            GrowthStage::Flowering => String::from("flowering"),
            GrowthStage::Fruiting => String::from("fruiting"),
            GrowthStage::Maturity => String::from("maturity"),
            GrowthStage::Harvest => String::from("harbest"),
            GrowthStage::Rot => String::from("rot"),
        }
    }

    fn get_days(&self) -> u32 {
        match self {
            GrowthStage::Seed => 3,
            GrowthStage::Seedling => 3,
            GrowthStage::Germination => 7,
            GrowthStage::Vegetative => 30,
            GrowthStage::Flowering => 20,
            GrowthStage::Fruiting => 15,
            GrowthStage::Maturity => 10,
            GrowthStage::Harvest => 5,
            GrowthStage::Rot => 0,
        }
    }

    pub fn next(&self, event: GrowthEvent) -> GrowthStage {
        match self {
            GrowthStage::Seed => match event {
                GrowthEvent::Sync => GrowthStage::Seedling,
                GrowthEvent::Fail => GrowthStage::Rot
            }
            GrowthStage::Seedling => match event {
                GrowthEvent::Sync => GrowthStage::Germination,
                GrowthEvent::Fail => GrowthStage::Rot
            },
            GrowthStage::Germination => match event {
                GrowthEvent::Sync => GrowthStage::Vegetative,
                GrowthEvent::Fail => GrowthStage::Rot
            },
            GrowthStage::Vegetative => match event {
                GrowthEvent::Sync => GrowthStage::Flowering,
                GrowthEvent::Fail => GrowthStage::Rot
            },
            GrowthStage::Flowering => match event {
                GrowthEvent::Sync => GrowthStage::Fruiting,
                GrowthEvent::Fail => GrowthStage::Rot
            },
            GrowthStage::Fruiting => match event {
                GrowthEvent::Sync => GrowthStage::Maturity,
                GrowthEvent::Fail => GrowthStage::Rot
            },
            GrowthStage::Maturity => match event {
                GrowthEvent::Sync => GrowthStage::Harvest,
                GrowthEvent::Fail => GrowthStage::Rot
            },
            _ => GrowthStage::Rot
        }
    }
}