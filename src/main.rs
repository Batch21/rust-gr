#[macro_use]
extern crate approx;

fn main() {
    println!("Hello, world!");
}

struct GR4JModel<'a> {
    rainfall: Vec<f64>,
    pet: Vec<f64>,
    production_store: &'a mut ProductionStore
}

impl GR4JModel<'_> {

    fn step(self, rainfall: f64, pet: f64) {
    }

}


struct ProductionStore {
    capacity: f64,
    water_content: f64,
}

impl ProductionStore {

    fn step(&mut self, rainfall: f64, pet: f64) -> f64 {
        
        let mut direct_to_routing = 0.0;

        if rainfall >= pet {
            let net_rainfall = rainfall - pet;
            let ps = self.rainfall_to_store(net_rainfall);
            direct_to_routing = net_rainfall - ps;
        } else {
            self.store_to_evap(pet - rainfall);
        }

        let perc = self.store_to_percolation();
        let total_to_routing = perc + direct_to_routing;
        total_to_routing

    }
    
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

    fn store_to_percolation(&mut self) -> f64 {
        let perc = self.water_content * (1.-(1.+(self.water_content/(9./4.*self.capacity)).powi(4)).powf(-0.25));
        self.water_content -= perc;
        perc
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
        assert!(abs_diff_eq!(store.water_content, 180.0 + 8.492, epsilon=0.001));
    }

    #[test]
    fn prod_store_pet() {

        let mut store = ProductionStore {
            capacity: 300.0,
            water_content: 240.0
        };

        store.store_to_evap(0.36);
        assert!(abs_diff_eq!(store.water_content, 240.0 - 0.3455, epsilon=0.001));      
    }

    #[test]
    fn prod_store_perc() {

        let mut store = ProductionStore {
            capacity: 300.0,
            water_content: 240.0
        };

        let perc = store.store_to_percolation();
        assert!(abs_diff_eq!(perc, 0.949, epsilon=0.001));
    }

    #[test]
    fn prod_store_step() {

        let mut store = ProductionStore {
            capacity: 300.0,
            water_content: 180.0
        };

        let to_routing = store.step(14.1, 0.46);
        assert!(abs_diff_eq!(to_routing, 5.4334, epsilon=0.001));
    }

    
}
 