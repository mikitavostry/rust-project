#[macro_use]
extern crate rocket;

use rand::Rng;

fn randomFromRange(from: i64, to: i64) -> i64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(from..to);
}

#[get("/calculateDisselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
fn calculateDiselUsage(distance: u32, yearOfProduction: u32, fuelUsagePer100KM: u32) -> String {
    let fuelUsage: f32 = (distance * fuelUsagePer100KM) as f32 / 100.0;
    format!("{}", fuelUsage)
}

#[get("/probabilityOfUnitInjectorFail?<VIN>")]
fn findProbability(VIN: String) -> String {
    format!("{}%", randomFromRange(0, 100))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![calculateDiselUsage, findProbability])
}
