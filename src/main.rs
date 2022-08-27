use rand::Rng;

use yew::prelude::*;

#[macro_use]
extern crate rocket;

fn randomFromRange(from: i64, to: i64) -> i64 {
    let mut rng = rand::thread_rng();
    let failProbability: i64 = rng.gen_range(from..to);
    return failProbability;
}

#[get("/calculateDisselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
fn calculateDiselusage(distance: u32, yearOfProduction: u32, fuelUsagePer100KM: u32) -> String {
    let fuelUsage: f32 = (distance * fuelUsagePer100KM) as f32 / 100.0;
    format!("Fuel consumption is {}.", fuelUsage)
}

#[get("/probabilityOfUnitInjectorFail?<VIN>")]
fn findProbability(VIN: String) -> String {
    format!(
        "The chance that the engine will fail is {}%.",
        randomFromRange(0, 100)
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![calculateDiselusage])
        .mount("/", routes![findProbability])
}
