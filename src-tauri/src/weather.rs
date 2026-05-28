use serde::Deserialize;

#[derive(Deserialize)]
struct CurrentWeather {
    temperature: f64,
}

#[derive(Deserialize)]
struct WeatherResponse {
    current_weather: CurrentWeather,
}

pub async fn get_weather(
    lat: f64,
    lon: f64,
) -> Result<String, String> {

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        lat,
        lon
    );

    let data: WeatherResponse =
        reqwest::get(url)
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())?;

    Ok(format!(
        "{}°C",
        data.current_weather.temperature
    ))
}