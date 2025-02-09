use std::f64::consts::PI;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0;
    let dlat = deg_to_rad(lat2 - lat1);
    let dlon = deg_to_rad(lon2 - lon1);
    
    let a = (dlat / 2.0).sin().powi(2) + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    r * c
}