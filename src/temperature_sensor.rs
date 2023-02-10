use rand::Rng;

const TEMP_SENSOR_RAW_MIN: i32 = 0;
// RAW minimum value of the sensor.
const TEMP_SENSOR_RAW_MAX: i32 = 1024;
// RAW maximum value of the sensor.
const TEMP_SENSOR_MIN: i32 = 0;
// Minimum range value in physical units (celcius)
const TEMP_SENSOR_MAX: i32 = 100; // Maximum range value in physical units (celcius)

pub struct TemperatureSensor {}

impl TemperatureSensor {
    fn m_raw_min() -> f32 {
        TEMP_SENSOR_RAW_MIN as f32
    }
    fn m_raw_max() -> f32 {
        TEMP_SENSOR_RAW_MAX as f32
    }
    fn m_min() -> f32 {
        TEMP_SENSOR_MIN as f32
    }
    fn m_max() -> f32 {
        TEMP_SENSOR_MAX as f32
    }
}

impl TemperatureSensor {
    pub fn get_current_reading(&self) -> f32 {
        let rand = rand::thread_rng().gen_range(120..900) as f32;
        let raw_reading = (rand
            % (TemperatureSensor::m_raw_max() - TemperatureSensor::m_raw_min())) + TemperatureSensor::m_raw_min();
        let result = ((TemperatureSensor::m_max() - TemperatureSensor::m_min())
            / (TemperatureSensor::m_raw_max() - TemperatureSensor::m_raw_min()))
            * (raw_reading);
        result
    }

    pub fn display(&self) {
        for _ in 0..10 {
            print!("Temperature: {}\n", self.get_current_reading());
        }
    }
}
