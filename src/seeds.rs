#![allow(dead_code)]

use crate::crop_parser::extract_content;


#[derive(Debug)]
pub enum SeedType {
    Sunflower(String, String),
    Pea(String, String),
    Carrot(String, String),
    Tomato(String, String),
    Broccoli(String, String),
}

impl SeedType {
    pub fn from_str(seed_type: &str) -> Result<SeedType, String> {
        match seed_type.to_lowercase().as_str() {
            "sun flower" => Ok(SeedType::Sunflower(seed_type.to_string(), "Botanica".to_string())),
            "pea" => Ok(SeedType::Pea(seed_type.to_string(), "Botanica".to_string())),
            "carrot" => Ok(SeedType::Carrot(seed_type.to_string(), "Botanica".to_string())),
            "tomato" => Ok(SeedType::Tomato(seed_type.to_string(), "Botanica".to_string())),
            "broccoli" => Ok(SeedType::Broccoli(seed_type.to_string(), "Botanica".to_string())),
            _ => Err("Invalid seed type".to_string()),
        }
    }

    pub fn get_botanica_name(&self) -> String {
        match self {
            SeedType::Sunflower(_, botanic_name) => botanic_name.to_string(),
            SeedType::Pea(_, botanic_name) => botanic_name.to_string(),
            SeedType::Carrot(_, botanic_name) => botanic_name.to_string(),
            SeedType::Tomato(_, botanic_name) => botanic_name.to_string(),
            SeedType::Broccoli(_, botanic_name) => botanic_name.to_string(),
        }
    }

    pub fn get_verbose_name(&self) -> String {
        match self {
            SeedType::Sunflower(verbose_name, _) => verbose_name.to_string(),
            SeedType::Pea(verbose_name, _) => verbose_name.to_string(),
            SeedType::Carrot(verbose_name, _) => verbose_name.to_string(),
            SeedType::Tomato(verbose_name, _) => verbose_name.to_string(),
            SeedType::Broccoli(verbose_name, _) => verbose_name.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct SeedBag {
    pub quantity: u32,
    pub seed_type: SeedType,
    pub species: String,
    pub is_gmo: Option<bool>,
    pub description: Option<String>,
}

impl SeedBag {
    pub fn new(quantity: u32, seed_type: &str, species: String) -> Option<Self> {
        let seed = SeedType::from_str(seed_type);
        match seed {
            Ok(seed) => Some(Self {
                quantity,
                seed_type: seed,
                species: species,
                description: None,
                is_gmo: None,
            }),
            Err(_) => panic!("Invalid seed type"),
        }
    }

    pub fn tear_bags() -> Vec<SeedBag> {
        let bag_maps = extract_content("test_data/seeds.csv");
        let mut bags: Vec<SeedBag> = Vec::new();

        for bag in bag_maps {
            let seed_type_str = bag.get("verbose_name").unwrap().to_string();
            let quantity_per_bag = bag.get("quantity_per_bag").unwrap().parse::<u32>().unwrap();
            let seed_bag = SeedBag::new(quantity_per_bag, &seed_type_str.as_str(), bag.get("species").unwrap().to_string());
            match seed_bag {
                Some(mut bg) => {
                    bg.is_gmo = Some(bag.get("is_gmo").unwrap().parse::<bool>().unwrap());
                    bg.description = Some(bag.get("description").unwrap().to_string());
                    bags.push(bg);
                },
                None => ()
            }
        }
        bags
    }


}