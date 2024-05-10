use std::time::Instant;

use log::trace;
use rand::seq::SliceRandom;

use crate::ails::vrp::{problem::VehicleRoutingProblem, routing_plan::InsertionHeuristic};

use super::vrp::routing_plan::RoutingPlan;

pub enum AILSPhase {
    PhaseOne,
    PhaseTwo,
}

pub struct AILS {}

impl AILS {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(
        &mut self,
        instance: &VehicleRoutingProblem,
        intial_solution: Option<RoutingPlan>,
        timeout: Instant,
    ) -> RoutingPlan {
        let initial_solution =
            intial_solution.unwrap_or_else(|| Self::find_initial_solution(instance));

        let mut reference_solution = initial_solution.clone();
        let mut optimal_solution = initial_solution.clone();

        while timeout.elapsed().as_secs() < 2 {
            let new_value = reference_solution.local_search(instance);

            if new_value < optimal_solution.value(instance) {
                trace!("Found new optimal: {}", new_value);
                optimal_solution = reference_solution.clone();
            }
        }

        return optimal_solution.clone();
    }

    fn find_initial_solution(instance: &VehicleRoutingProblem) -> RoutingPlan {
        let total_demand = (0..instance.number_of_customers)
            .map(|client| instance.demand(client))
            .sum::<u32>();
        let number_of_routes =
            ((total_demand + instance.vehicle_capacity - 1) / instance.vehicle_capacity) as usize;
        assert_eq!(number_of_routes, instance.number_of_vehicles);

        let mut clients = (0..instance.number_of_customers).collect::<Vec<_>>();
        let mut routes = vec![vec![]; number_of_routes];

        let random_clients: Vec<usize> = clients
            .choose_multiple(&mut rand::thread_rng(), number_of_routes)
            .cloned()
            .collect();

        for (route, client) in routes.iter_mut().zip(random_clients) {
            clients.retain(|&c| c != client);
            route.push(client);
        }

        let mut routing_plan = RoutingPlan::new(routes);

        for client in clients {
            routing_plan.insert(instance, client, InsertionHeuristic::Distance)
        }

        routing_plan
    }
}
