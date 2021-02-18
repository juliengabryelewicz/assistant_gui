use json::JsonValue;
use std::error::Error;

pub fn get_weather_from_search(location: &str, api_key: &str)  -> Result<JsonValue, Box<dyn Error>> {
    let url1 = "http://api.openweathermap.org/data/2.5/weather?q=";
    let url2 = "&appid=";
    let url = [url1, location, url2, api_key].concat();
    let resp = reqwest::blocking::get(&url)?.text()?;
    Ok(json::parse(&resp).unwrap())
}

pub fn calculate_temperature(temperature: f32)  -> String {
    (temperature - 273.15).floor().to_string()
}