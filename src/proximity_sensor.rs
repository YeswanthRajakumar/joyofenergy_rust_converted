use rand::Rng;

pub struct ProximitySensor {}

const PROXIMITY_BIT_INDEX: i32 = 4;

impl ProximitySensor {
    pub fn get_proximity_state(&self) -> i32 {
        let r = rand::thread_rng().gen_range(120..900);
        ((r & (1 << PROXIMITY_BIT_INDEX)) > 0) as i32
    }

        pub fn display(&self) {
        for _ in 0..10 {
            print!("Proximity: {}\n", self.get_proximity_state());
        }
    }
}
