#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub enum WeatherCondition {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
    Stormy
}

impl Default for WeatherCondition {
    fn default() -> Self {
        Self::Sunny
    }
}

impl WeatherCondition {
    fn get_weather_condition(&self) -> String {
        match self {
            WeatherCondition::Sunny => String::from("sunny"),
            WeatherCondition::Cloudy => String::from("cloudy"),
            WeatherCondition::Rainy => String::from("rainy"),
            WeatherCondition::Snowy => String::from("snowy"),
            WeatherCondition::Stormy => String::from("stormy"),
        }
    }

    fn from_str(weather_condition: &str) -> Option<Self> {
        match weather_condition.to_lowercase().as_str() {
            "sunny" => Some(WeatherCondition::Sunny),
            "cloudy" => Some(WeatherCondition::Cloudy),
            "rainy" => Some(WeatherCondition::Rainy),
            "snowy" => Some(WeatherCondition::Snowy),
            "stormy" => Some(WeatherCondition::Stormy),
            _ => Some(WeatherCondition::Sunny),
        }
    }

    pub fn generate_random_weather_condition() -> WeatherCondition {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..=4);
        match random {
            0 => WeatherCondition::Sunny,
            1 => WeatherCondition::Cloudy,
            2 => WeatherCondition::Rainy,
            3 => WeatherCondition::Snowy,
            4 => WeatherCondition::Stormy,
            _ => WeatherCondition::Sunny,
        }
    }
}

#[derive(Debug)]
struct WeatherData {
    temperature: f32,
    humidity: f32,
    weather_condition: WeatherCondition,
    humidity_max: Option<f32>,
    wind_speed: Option<f32>,
    rain_probability: Option<f32>,
    snow_probability: Option<f32>,
    cloud_cover: Option<f32>,
    description: Option<String>,
    precipitation: Option<f32>,
    visibility: Option<f32>,
    pressure: Option<f32>,
    metadata: Option<WeatherMetadata>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WeatherMetadata {
    latitude: f32,
    longitude: f32,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
    feelslike_max: Option<f32>,
    feelslike_min: Option<f32>,
    feelslike: Option<f32>,
}

impl WeatherData {

}