// src/weather.rs


use serde::Deserialize; // deserialize json

#[derive(Deserialize)]
struct CurrentWeather {temperature: f64}

// weather response struct
#[derive(Deserialize)]
struct WeatherResponse {current_weather: CurrentWeather}

// get weather function
pub async fn get_weather(lat: f64, lon: f64) -> Result<String, String> { // get weather from open-meteo api

    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true", lat, lon); // api url

    let data: WeatherResponse = // reqest used get request and parse json
        reqwest::get(url)
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

    Ok(format!("{}°C", data.current_weather.temperature)) // return temperature
}