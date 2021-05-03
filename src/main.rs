#[macro_use]
extern crate approx;

fn main() {
    println!("Hello, world!");
}

struct GR4JModel<'a> {
    raifall: Vec<f64>,
    pet: Vec<f64>,
    production_store: &'a mut ProductionStore
}

impl GR4JModel<'_> {

    fn step(self) {

    }

    fn update_production_store(self, rainfall: f64, pet: f64) {
        if rainfall >= pet {
            let ps = self.production_store.rainfall_to_store(rainfall - pet);
        } else {
            self.production_store.store_to_evap(pet - rainfall);
        }
    }

}


struct ProductionStore {
    capacity: f64,
    water_content: f64,
}

impl ProductionStore {
    
    fn rainfall_to_store(&mut self, net_rainfall: f64) -> f64{
        let tws = (net_rainfall / self.capacity).tanh();
        let sr = self.water_content / self.capacity;
        let ps = self.capacity * (1.0 - sr.powi(2)) * tws / (1.0 + sr * tws);
        self.water_content += ps;
        ps
    }

    fn store_to_evap(&mut self, net_evap_capacity: f64) {
        let ws = (net_evap_capacity / self.capacity).tanh();
        let sr = self.water_content / self.capacity;
        let er =  self.water_content * (2.0 - sr) * ws / (1.0 + (1.0 - sr) * ws);
        self.water_content -= er;
    }
}

struct Routing {
    days: f64,
    exhange_coefficient: f64
}

struct RoutingStore {
    capacity: f64,
    water_content: f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prod_store_rainfall() {

        let mut store = ProductionStore {
            capacity: 300.0,
            water_content: 180.0
        };

        store.rainfall_to_store(13.64);
        println!("{}", store.water_content);
        assert!(abs_diff_eq!(store.water_content, 180.0 + 8.492, epsilon=0.001));
    }

    #[test]
    fn prod_store_pet() {

        let mut store = ProductionStore {
            capacity: 300.0,
            water_content: 240.0
        };

        store.store_to_evap(0.36);
        println!("{}", 240.0 - store.water_content);
        assert!(abs_diff_eq!(store.water_content, 240.0 - 0.3455, epsilon=0.001));
    }
    
}
 