use crate::utils::{s_curve1, s_curve2};
pub struct ProductionStore {
    capacity: f64,
    water_content: f64,
}

impl ProductionStore {
    pub fn step(&mut self, rainfall: f64, pet: f64) -> f64 {
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

    fn rainfall_to_store(&mut self, net_rainfall: f64) -> f64 {
        let tws = (net_rainfall / self.capacity).tanh();
        let sr = self.water_content / self.capacity;
        let ps = self.capacity * (1.0 - sr.powi(2)) * tws / (1.0 + sr * tws);
        self.water_content += ps;
        ps
    }

    fn store_to_evap(&mut self, net_evap_capacity: f64) {
        let ws = (net_evap_capacity / self.capacity).tanh();
        let sr = self.water_content / self.capacity;
        let er = self.water_content * (2.0 - sr) * ws / (1.0 + (1.0 - sr) * ws);
        self.water_content -= er;
    }

    fn store_to_percolation(&mut self) -> f64 {
        let perc = self.water_content
            * (1. - (1. + (self.water_content / (9. / 4. * self.capacity)).powi(4)).powf(-0.25));
        self.water_content -= perc;
        perc
    }

    pub fn new(
        capacity: f64,
        water_content: f64
    ) -> ProductionStore {

        ProductionStore {
            capacity: capacity,
            water_content: water_content
        }
    }
}

pub struct Routing {
    store: RoutingStore,
    uh1_ordinates: Vec<f64>,
    uh2_ordinates: Vec<f64>,
    uh1: Vec<f64>,
    uh2: Vec<f64>,
}

impl Routing {
    pub fn step(&mut self, to_routing: f64) -> f64 {
        // convolution of first unit hydrograph
        for i in 0..self.uh1.len() - 1 {
            self.uh1[i] = self.uh1[i + 1] + to_routing * self.uh1_ordinates[i];
        }
        self.uh1[self.uh1_ordinates.len() - 1] = to_routing * self.uh1_ordinates.last().unwrap();

        // convolution of second unit hydrograph
        for i in 0..self.uh2.len() - 1 {
            self.uh2[i] = self.uh2[i + 1] + to_routing * self.uh2_ordinates[i];
        }
        self.uh2[self.uh2_ordinates.len() - 1] = to_routing * self.uh2_ordinates.last().unwrap();

        let (qr, exchange) = self.store.step(self.uh1[0] * 0.9);

        let mut qd = self.uh2[0] * 0.1 + exchange;
        qd = qd.max(0.0);

        qr + qd
    }

    pub fn new(
        days: f64,
        exchange_coefficient: f64,
        store_capacity: f64,
        store_content: f64,
    ) -> Routing {
        let num_ords1 = days.ceil() as i32;
        let num_ords2 = (days * 2.0).ceil() as i32;

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

        let store = RoutingStore {
            capacity: store_capacity,
            water_content: store_content,
            gw_exchange_coefficient: exchange_coefficient,
        };

        Routing {
            store: store,
            uh1_ordinates: uh1_ords,
            uh2_ordinates: uh2_ords,
            uh1: vec![0.0; num_ords1 as usize],
            uh2: vec![0.0; num_ords2 as usize],
        }
    }
}

struct RoutingStore {
    capacity: f64,
    water_content: f64,
    gw_exchange_coefficient: f64,
}

impl RoutingStore {
    fn step(&mut self, uh1_output: f64) -> (f64, f64) {
        let exchange =
            self.gw_exchange_coefficient * (self.water_content / self.capacity).powf(3.5);
        self.water_content = self.water_content + uh1_output + exchange;
        self.water_content = self.water_content.max(0.0);
        let qr = self.water_content
            * (1.0 - (1.0 + (self.water_content / self.capacity).powi(4)).powf(-0.25));
        self.water_content -= qr;
        (qr, exchange)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_production_store_rainfall() {
        let mut store = ProductionStore {
            capacity: 300.0,
            water_content: 180.0,
        };

        store.rainfall_to_store(13.64);
        assert!(abs_diff_eq!(
            store.water_content,
            180.0 + 8.492,
            epsilon = 0.001
        ));
    }

    #[test]
    fn test_production_store_pet() {
        let mut store = ProductionStore {
            capacity: 300.0,
            water_content: 240.0,
        };

        store.store_to_evap(0.36);
        assert!(abs_diff_eq!(
            store.water_content,
            240.0 - 0.3455,
            epsilon = 0.001
        ));
    }

    #[test]
    fn test_production_store_perc() {
        let mut store = ProductionStore {
            capacity: 300.0,
            water_content: 240.0,
        };

        let perc = store.store_to_percolation();
        assert!(abs_diff_eq!(perc, 0.949, epsilon = 0.001));
    }

    #[test]
    fn test_production_store_step() {
        let mut store = ProductionStore {
            capacity: 300.0,
            water_content: 180.0,
        };

        let to_routing = store.step(14.1, 0.46);
        assert!(abs_diff_eq!(to_routing, 5.4334, epsilon = 0.001));
    }

    #[test]
    fn test_routing() {
        let mut routing = Routing::new(1.5, 2.5, 70.0, 49.0);

        let ord1_expected = vec![0.3629, 0.6371];
        for (i, ord) in routing.uh1_ordinates.iter().enumerate() {
            assert!(abs_diff_eq!(ord.clone(), ord1_expected[i], epsilon = 0.001));
        }

        let ord2_expected = vec![0.1814, 0.6371, 0.1814];
        for (i, ord) in routing.uh2_ordinates.iter().enumerate() {
            assert!(abs_diff_eq!(ord.clone(), ord2_expected[i], epsilon = 0.001));
        }

        let q = routing.step(5.4335);

        assert!(abs_diff_eq!(q, 4.018, epsilon = 0.001));
    }

    #[test]
    fn test_routing_store() {
        let mut store = RoutingStore {
            capacity: 70.0,
            water_content: 49.0,
            gw_exchange_coefficient: 2.5,
        };

        let (qr, exchange) = store.step(1.7746);

        assert!(abs_diff_eq!(qr, 3.202, epsilon = 0.001));
        assert!(abs_diff_eq!(exchange, 0.717, epsilon = 0.001));
    }

}