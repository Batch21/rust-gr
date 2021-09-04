use crate::components::{ProductionStore, Routing};
use crate::data::Parameters;

pub struct GR4JModel {
    production_store: ProductionStore,
    routing: Routing,
}

impl GR4JModel {
    pub fn run(&mut self, rainfall: Vec<f64>, pet: Vec<f64>) -> Vec<f64> {
        let mut simulated = Vec::new();

        for (r, p) in rainfall.iter().zip(pet) {
            let q = self.step(*r, p);
            simulated.push(q)
        }
        simulated
    }

    fn step(&mut self, rainfall: f64, pet: f64) -> f64 {
        let to_routing = self.production_store.step(rainfall, pet);
        let q = self.routing.step(to_routing);
        q
    }

    pub fn new(parameters: Parameters) -> GR4JModel {
        let production_store = ProductionStore::new(
            parameters.production_store_capacity,
            parameters.production_store_content,
        );

        let routing = Routing::new(
            parameters.days,
            parameters.exchange_coefficient,
            parameters.routing_store_capacity,
            parameters.routing_store_content,
        );

        GR4JModel {
            production_store: production_store,
            routing: routing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gr4j() {
        let parameters = Parameters::new(300.0, 2.5, 70.0, 1.5, 180.0, 49.0);

        let mut model = GR4JModel::new(parameters);

        let rainfall = vec![14.1, 3.7, 7.1, 9.3, 7.1];
        let pet = vec![0.46, 0.46, 0.47, 0.47, 0.48];

        let sim = model.run(rainfall, pet);

        let expected = vec![4.018, 4.574, 4.240, 4.397, 4.721];

        for (s, e) in sim.iter().zip(expected) {
            assert!(abs_diff_eq!(s.clone(), e, epsilon = 0.001));
        }
    }
}
