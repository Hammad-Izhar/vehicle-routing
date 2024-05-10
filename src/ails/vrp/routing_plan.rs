use std::collections::HashSet;

use crate::ails::vrp::client::ClientId;
use crate::ails::vrp::problem::VehicleRoutingProblem;

use super::client;

pub enum InfeasibleError {
    TooManyVehicles,
    ExceedsVehicleCapacity,
    InvalidTour,
}

pub enum LocalSearchError {
    NoImprovement,
    InvalidArguments,
    InvalidTour,
}

pub struct RoutingPlan {
    pub routes: Vec<Vec<ClientId>>,
}

impl RoutingPlan {
    /// Represents a potential solution to the VRP.
    /// Parameters:
    ///     routes: A list of routes, where each route is a list of client IDs
    ///             The depot is implicitly assumed to be the first and last client of each route
    ///     problem: The associated VRP problem
    pub fn new(routes: Vec<Vec<ClientId>>) -> Self {
        RoutingPlan { routes }
    }

    pub fn feasible(&self, problem: &VehicleRoutingProblem) -> Result<(), InfeasibleError> {
        if self.routes.len() > problem.number_of_vehicles {
            return Err(InfeasibleError::TooManyVehicles);
        }

        let customers_visited: HashSet<&usize> = self.routes.iter().flatten().collect();
        if customers_visited.len() != problem.number_of_customers {
            return Err(InfeasibleError::InvalidTour);
        }

        for route in &self.routes {
            let route_demand = route.iter().map(|c| problem.demand(*c)).sum::<u32>();
            if route_demand > problem.vehicle_capacity {
                return Err(InfeasibleError::ExceedsVehicleCapacity);
            }
        }

        Ok(())
    }

    pub fn inter_shift(&mut self, client: ClientId) {}

    pub fn inter_swap(
        &mut self,
        client1: ClientId,
        client2: ClientId,
    ) -> Result<(), LocalSearchError> {
        let mut client1_route = None;
        let mut client2_route = None;
        for (route_idx, route) in self.routes.iter_mut().enumerate() {
            if route.contains(&client1) {
                client1_route = Some(route_idx);
            }
            if route.contains(&client2) {
                client2_route = Some(route_idx);
            }
        }

        match (client1_route, client2_route) {
            (Some(client1_route), Some(client2_route)) => {
                if client1_route == client2_route {
                    return Err(LocalSearchError::InvalidArguments);
                }

                let client1_idx = self.routes[client1_route]
                    .iter()
                    .position(|c| c == &client1)
                    .unwrap();
                let client2_idx = self.routes[client2_route]
                    .iter()
                    .position(|c| c == &client2)
                    .unwrap();

                self.routes[client1_route].remove(client1_idx);
                self.routes[client2_route].remove(client2_idx);

                let mut best_index = 0;
                let mut best_cost = f64::INFINITY;

                Ok(())
            }
            _ => Err(LocalSearchError::InvalidTour),
        }
    }

    pub fn inter_cross(&mut self, client1: ClientId, client2: ClientId) {}

    pub fn intra_shift(&mut self, client: ClientId) {}

    pub fn intra_swap(
        &mut self,
        client1: ClientId,
        client2: ClientId,
    ) -> Result<(), LocalSearchError> {
        let mut client1_route = None;
        let mut client2_route = None;
        for (route_idx, route) in self.routes.iter_mut().enumerate() {
            if route.contains(&client1) {
                client1_route = Some(route_idx);
            }
            if route.contains(&client2) {
                client2_route = Some(route_idx);
            }
        }

        match (client1_route, client2_route) {
            (Some(client1_route), Some(client2_route)) => {
                if client1_route != client2_route {
                    return Err(LocalSearchError::InvalidArguments);
                }

                let client1_idx = self.routes[client1_route]
                    .iter()
                    .position(|c| c == &client1)
                    .unwrap();
                let client2_idx = self.routes[client2_route]
                    .iter()
                    .position(|c| c == &client2)
                    .unwrap();

                if client1_idx == client2_idx {
                    return Err(LocalSearchError::InvalidArguments);
                }

                self.routes[client1_route].swap(client1_idx, client2_idx);

                Ok(())
            }
            _ => Err(LocalSearchError::InvalidTour),
        }
    }

    pub fn intra_cross(&mut self, client1: ClientId, client2: ClientId) {}
}
