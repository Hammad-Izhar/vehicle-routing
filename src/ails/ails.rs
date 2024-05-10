use std::collections::HashSet;

use rand::seq::SliceRandom;

use crate::ails::vrp::problem::VehicleRoutingProblem;

use super::vrp::routing_plan::RoutingPlan;

pub enum AILSPhase {
    PhaseOne,
    PhaseTwo,
}

struct AILS {
    phase: AILSPhase,
    instance: VehicleRoutingProblem,
    optimal_solution: RoutingPlan,
    reference_solution: RoutingPlan,
}

impl AILS {
    pub fn new(instance: VehicleRoutingProblem) -> Self {
        let routing_plan = find_initial_solution(&instance);

        Self {
            phase: AILSPhase::PhaseOne,
            instance,
            optimal_solution: routing_plan.clone(),
            reference_solution: routing_plan,
        }
    }

    pub fn run(&mut self) {}

    fn find_initial_solution(instance: &VehicleRoutingProblem) -> RoutingPlan {
        let total_demand = (0..instance.number_of_customers)
            .map(|client| instance.demand(client))
            .sum::<u32>();
        let number_of_routes =
            ((total_demand + instance.vehicle_capacity - 1) / instance.vehicle_capacity) as usize;

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

        let routing_plan = RoutingPlan::new(routes);

        for client in clients {
            routing_plan.insert()
        }

        routing_plan
    }
}
