use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

#[derive(Deserialize)]
struct CurrentCondition {
    #[serde(rename = "weatherDesc")]
    weather_desc: Vec<HashMap<String, String>>,
}

#[derive(Deserialize)]
struct WeatherResponse {
    current_condition: Vec<CurrentCondition>,
}

impl WeatherResponse {
    fn get_weather_desc(&self) -> String {
        self.current_condition[0].weather_desc[0]
            .get("value")
            .unwrap()
            .clone()
    }
}

pub fn get_weather_from_body(body: &str) -> Result<String, String> {
    match serde_json::from_str::<WeatherResponse>(body) {
        Ok(response) => Ok(response.get_weather_desc()),
        Err(e) => Err(e.to_string()),
    }
}
