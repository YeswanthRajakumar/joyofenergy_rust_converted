use humidity_sensor::HumiditySensor;
use proximity_sensor::ProximitySensor;
use temperature_sensor::TemperatureSensor;

mod humidity_sensor;
mod proximity_sensor;
mod temperature_sensor;

trait Readable{
    fn get_reading(&self);
}

fn main() {
    let t = TemperatureSensor {};
    let p = ProximitySensor {};
    let h = HumiditySensor {};

    t.display();
    p.display();
    h.display();

    // for i in 0..10 {
    //     print!("-> {} ", i);
    //     print!(", Proximity: {}", p.get_proximity_state());
    //     println!(", Humidity: {}", h.get_current_reading());
    // }
}
