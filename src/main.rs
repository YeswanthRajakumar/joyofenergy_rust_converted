use humidity_sensor::HumiditySensor;
use proximity_sensor::ProximitySensor;
use temperature_sensor::TemperatureSensor;

mod proximity_sensor;
mod humidity_sensor;
mod temperature_sensor;


fn main() {
    let t = TemperatureSensor {};
    let p = ProximitySensor {};
    let h = HumiditySensor {};

    for i in 0..10 {
        print!("-> {} ", i);
        print!("Temperature: {}", t.GetCurrentReading());
        print!(", Proximity: {}", p.GetProximityState());
        println!(", Humidity: {}", h.GetCurrentReading());
    }
}
