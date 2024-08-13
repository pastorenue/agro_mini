trait Harvestable {
    fn get_harvest(&self) -> String;
    fn get_yield(&self) -> u32;
    fn is_harvestable(&self) -> bool;
    fn harvest(&self) -> ();
}

trait Sowable {
    fn process_seedlings(&self) -> ();
    fn get_seedlings(&self) -> Vec<String>;
    fn apply_fertilizer(&self) -> ();
    fn apply_water(&self) -> ();
    fn fumigate_seedlings(&self) -> ();
    fn is_due(&self) -> bool;
}

trait Farmable: Harvestable + Sowable {
    fn manage_farm(&self) -> ();
}

trait FarmSpec {
    fn get_area(&self) -> u32;
    fn get_crops(&self) -> Vec<String>;
    fn get_soil_type(&self) -> String;
}
