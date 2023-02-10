use rand::Rng;

const HUMIDITY_RAW_MIN: i32 = 0;
// RAW minimum value of the sensor.
const HUMIDITY_RAW_MAX: i32 = 1024;
// RAW maximum value of the sensor.
const HUMIDITY_MIN: i32 = 0;
// Minimum range value in physical units (celcius)
const HUMIDITY_MAX: i32 = 100; // Maximum range value in physical units (celcius)

pub struct HumiditySensor {}

impl HumiditySensor {
    fn m_raw_min() -> f32 {
        HUMIDITY_RAW_MIN as f32
    }
    fn m_raw_max() -> f32 {
        HUMIDITY_RAW_MAX as f32
    }
    fn m_min() -> f32 {
        HUMIDITY_MIN as f32
    }
    fn m_max() -> f32 {
        HUMIDITY_MAX as f32
    }
}

impl HumiditySensor {
    pub fn get_current_reading(&self) -> f32 {
        let rand = rand::thread_rng().gen_range(120..900) as f32;
        let raw_reading = rand % (HumiditySensor::m_raw_max() - HumiditySensor::m_raw_min())
            + HumiditySensor::m_raw_min();
        ((HumiditySensor::m_max() - HumiditySensor::m_min())
            / (HumiditySensor::m_raw_max() - HumiditySensor::m_raw_min()))
            * (raw_reading)
    }

        pub fn display(&self) {
        for _ in 0..10 {
            print!("Humidity: {}\n", self.get_current_reading());
        }
    }
}
