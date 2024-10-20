use std::env;
use std::io;
use dotenv::dotenv;

mod api;

#[tokio::main]
async fn main(){
    dotenv().ok();
    let mut input = String::new();
    println!("Please enter City Name:");
    io::stdin().read_line(&mut input).expect("Failed");


    let city = input.trim();
    let api_key = env::var("API_KEY").expect("API_KEY not set in .env file");

    match api::get_forcast(city, &api_key).await {
        
        Ok(forcast)=>{
            println!("Temperature: {} C", forcast.main.temp );
            for weather in forcast.weather{
                println!("Describe: {}", weather.description);

            }
        }
        
        Err(e)=>{
            println!("Error fetching forcast: {:?}", e);
        }

    }
}