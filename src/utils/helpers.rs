pub fn location_split(location: &str) -> (String, String) {
    // Divide provided location into system and waypoint coords
    // input X1-DF55-20250Z to system: X1-DF55 and waypoint: 20250Z
    let location_split: Vec<&str> = location.split('-').collect();
    let system = format!(
        "{}-{}",
        location_split[0].trim_matches('"'),
        location_split[1].trim_matches('"')
    );
    let waypoint = location.trim_matches('"').to_string();

    (system, waypoint)
}
