use dotenv::dotenv;
use futures::future::join_all;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::env;

use crate::utils::ui;

#[derive(Debug)]
#[allow(dead_code)]
struct WeatherApiError {
    message: String,
    status: u16,
    body: String,
}

impl WeatherApiError {
    fn new(message: String, status: u16, body: String) -> Self {
        Self {
            message,
            status,
            body,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GeoSearchResponse {
    name: String,
    lat: f64,
    lon: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct WeatherData {
    weather: Vec<Weather>,
    main: Main,
    name: String,
}

struct OpenWeatherApi {
    base_url: String,
    api_key: String,
}

impl OpenWeatherApi {
    fn new() -> Self {
        dotenv().ok();
        Self {
            base_url: env::var("OPEN_WEATHER_API_URL").unwrap(),
            api_key: env::var("OPEN_WEATHER_API_KEY").unwrap(),
        }
    }

    async fn get_request<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &[(&str, &String)],
    ) -> Result<T, WeatherApiError> {
        let url = format!("{}{}", self.base_url, path);
        let client = Client::new();
        let response = client.get(url).query(params).send().await;
        if response.is_err() {
            let err = response.err().unwrap();
            return Err(WeatherApiError::new(
                "Failed to get request".to_string(),
                err.status().unwrap().as_u16(),
                err.to_string(),
            ));
        }
        match response.unwrap().error_for_status() {
            Ok(response) => {
                let json = response.json::<T>().await;
                if json.is_err() {
                    return Err(WeatherApiError::new(
                        "Failed to parse response".to_string(),
                        200,
                        json.err().unwrap().to_string(),
                    ));
                }
                Ok(json.unwrap())
            }
            Err(e) => Err(WeatherApiError::new(
                e.to_string(),
                e.status().unwrap().into(),
                e.to_string(),
            )),
        }
    }
    async fn geo_search(
        &self,
        city: &str,
        state: &str,
        country: &str,
    ) -> Result<Vec<GeoSearchResponse>, WeatherApiError> {
        let path = "/geo/1.0/direct";
        let params = [
            ("q", &format!("{},{},{}", city, state, country)),
            ("limit", &"5".to_string()),
            ("appid", &self.api_key),
        ];
        self.get_request(path, &params).await
    }
    async fn weather_data(&self, lat: f64, lon: f64) -> Result<WeatherData, WeatherApiError> {
        let path = "/data/2.5/weather";
        let params = [
            ("lat", &lat.to_string()),
            ("lon", &lon.to_string()),
            ("appid", &self.api_key),
            ("units", &"metric".to_string()),
        ];
        self.get_request(path, &params).await
    }
    async fn search_weather_for_location(
        &self,
        location: GeoSearchResponse,
    ) -> Result<WeatherData, WeatherApiError> {
        self.weather_data(location.lat, location.lon).await
    }
    async fn search_weather_for_locations(
        &self,
        locations: Vec<GeoSearchResponse>,
    ) -> Vec<WeatherData> {
        let futures = locations.iter().map(|location| {
            let location = location.clone();
            async move { self.search_weather_for_location(location).await }
        });
        join_all(futures)
            .await
            .into_iter()
            .map(|x| x.unwrap())
            .collect()
    }
}

#[tokio::main]
pub async fn main() {
    let api = OpenWeatherApi::new();
    let mut locations: Vec<GeoSearchResponse> = vec![];
    let iteration_actions = vec!["Search for a location", "Continue"];

    loop {
        if !locations.is_empty() {
            let selection = ui::ui_select("What next?", iteration_actions.clone());
            match selection {
                1 => break,
                _ => (),
            }
        }

        let city = ui::get_input::<String>("Enter a city");
        let _locations = api.geo_search(&city, "", "TR").await.unwrap();
        if _locations.is_empty() {
            println!("No locations found");
            continue;
        }
        let location = _locations.first().unwrap();
        locations.push(location.clone());
    }

    let weather_data = api.search_weather_for_locations(locations).await;
    weather_data.iter().for_each(|weather| {
        let weather_icon = match weather.weather.first().unwrap().description.as_str() {
            "clear sky" => "☀️",
            "few clouds" => "🌤",
            "scattered clouds" => "☁️",
            "broken clouds" => "☁️☁️",
            "shower rain" => "🌧",
            "rain" => "🌧",
            "thunderstorm" => "⚡️",
            "snow" => "❄️",
            "mist" => "🌫",
            _ => "🌤",
        };
        println!(
            "{}: {} {}",
            weather.name, weather_icon, weather.main.temp_max
        );
    });
}
