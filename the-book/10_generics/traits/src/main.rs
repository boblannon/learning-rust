extern crate traits;

use traits::Summarizable;
use traits::notify;

struct WeatherForecast {
    high_temp: f64,
    low_temp: f64,
    chance_of_precipitation: f64,
}

impl Summarizable for WeatherForecast {
    fn summary(&self) -> String {
        format!("The high will be {}, and the low will be {}. The chance of precipitiation is {}%.",
                self.high_temp, self.low_temp, self.chance_of_precipitation)
    }

    fn author_summary(&self) -> String {
        format!("NWCC")
    }
}

fn main() {
    let forecast = WeatherForecast { high_temp: 100.0, low_temp: 85.0, chance_of_precipitation: 35.0 };
    println!("{}", forecast.summary());

    notify(forecast);

    // won't work on any type that doesn't impl Summarizable, so this won't compile:
    //notify(String::from("Hello!"));
}
