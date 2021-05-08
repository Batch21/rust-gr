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
    exchange_coefficient: f64,
    uh1_ordinates: Vec<f64>,
    uh2_ordinates: Vec<f64>,
    uh1: Vec<f64>,
    uh2: Vec<f64>
}

impl Routing {

    fn new(days: f64, exchange_coefficient: f64) -> Routing {
        let num_ords1 = days.ceil() as i32;
        let num_ords2 = (days * 2.0).ceil() as i32;
        
        println!("ords1: {}", num_ords1);
        println!("ords2 {}", num_ords2);

        let mut uh1_ords = Vec::new();
        for i in 0..num_ords1 {
            let t = i as f64;
            uh1_ords.push(s_curve1(t + 1.0, days) - s_curve1(t, days));
        }

        let mut uh2_ords = Vec::new();
        for i in 0..num_ords2 {
            let t = i as f64;
            uh2_ords.push(s_curve2(t + 1.0, days) - s_curve2(t, days));
        }

        Routing {
            days: days,
            exchange_coefficient: exchange_coefficient,
            uh1_ordinates: uh1_ords,
            uh2_ordinates: uh2_ords,
            uh1: Vec::new(),
            uh2: Vec::new()
        }
    }

}

fn s_curve1(t: f64, days: f64) -> f64 {
    if t <= 0.0 {
        return 0.0 
    }
    else if t < days {
        return (t/days).powf(2.5)
    }
    1.0
}

fn s_curve2(t: f64, days: f64) -> f64 {
    if t <= 0.0 {
        return 0.0
    }   
    else if t < days{
        return 0.5*(t/days).powf(2.5)   
    }   
    else if t < 2.0 * days{
        return 1.0 - 0.5*(2.0 - t/days).powf(2.5) 
    }
    1.0
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

    #[test]
    fn routing() {

        let routing = Routing::new(1.5, 200.0);

        let ord1_expected = vec![0.3629, 0.6371];
        for (i, ord) in routing.uh1_ordinates.iter().enumerate(){
            assert!(abs_diff_eq!(ord.clone(), ord1_expected[i], epsilon=0.001));
        } 
        
        let ord2_expected = vec![0.1814, 0.6371, 0.1814];
        for (i, ord) in routing.uh2_ordinates.iter().enumerate(){
            assert!(abs_diff_eq!(ord.clone(), ord2_expected[i], epsilon=0.001));
        }     
    }

    
}
 