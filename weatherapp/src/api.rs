use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ForcastData{
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
pub struct Main{
    pub temp: f64,
}

#[derive(Deserialize, Debug)]
pub struct Weather{
    pub description: String,
}

pub async fn get_forcast(city: &str, api_key: &str) -> Result<ForcastData, Error>{
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", city, api_key);
    let res = reqwest::get(&url).await?.json::<ForcastData>().await?;
    Ok(res)
}