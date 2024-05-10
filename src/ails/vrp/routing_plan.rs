use std::cmp::min;
use std::collections::HashSet;

use log::trace;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;

use crate::ails::vrp::client::ClientId;
use crate::ails::vrp::problem::VehicleRoutingProblem;

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

pub enum RemovalHeuristic {
    Concentric,
    Sequential,
}

pub enum InsertionHeuristic {
    Cost,
    Distance,
}

pub enum SearchStrategy {
    InterShift,
    InterSwap,
    IntraShift,
    IntraSwap,
}

#[derive(Debug, Clone)]
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

    pub fn value(&self, instance: &VehicleRoutingProblem) -> f64 {
        let mut total_distance = 0.0;
        for route in &self.routes {
            for (i, client) in route.iter().enumerate() {
                let prev_client = if i == 0 { 0 } else { route[i - 1] };

                total_distance += instance.graph.distance(*client, prev_client);
            }
            total_distance += instance.graph.distance(*route.last().unwrap(), 0);
        }

        total_distance
    }

    pub fn local_search(&mut self, instance: &VehicleRoutingProblem) -> f64 {
        let strategy = [
            SearchStrategy::InterShift,
            SearchStrategy::InterSwap,
            SearchStrategy::IntraShift,
            SearchStrategy::IntraSwap,
        ]
        .choose(&mut rand::thread_rng())
        .unwrap();

        match strategy {
            SearchStrategy::InterShift => {
                trace!("Modifying route using InterShift");
                let client = self
                    .routes
                    .iter()
                    .flatten()
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                self.inter_shift(instance, *client);
            }
            SearchStrategy::InterSwap => {
                trace!("Modifying route using InterSwap");
                let client1 = self
                    .routes
                    .iter()
                    .flatten()
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                let client2 = self
                    .routes
                    .iter()
                    .flatten()
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                self.inter_swap(instance, *client1, *client2);
            }
            SearchStrategy::IntraShift => {
                trace!("Modifying route using IntraShift");
                let client = self
                    .routes
                    .iter()
                    .flatten()
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                self.intra_shift(instance, *client);
            }
            SearchStrategy::IntraSwap => {
                trace!("Modifying route using IntraSwap");
                let client1 = self
                    .routes
                    .iter()
                    .flatten()
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                let client2 = self
                    .routes
                    .iter()
                    .flatten()
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                self.intra_swap(*client1, *client2);
            }
        };

        self.value(&instance)
    }

    pub fn insert(
        &mut self,
        instance: &VehicleRoutingProblem,
        client: ClientId,
        method: InsertionHeuristic,
    ) {
        match method {
            InsertionHeuristic::Cost => self.insert_cost(instance, client),
            InsertionHeuristic::Distance => self.insert_distance(instance, client),
        }
    }

    fn insert_cost(&mut self, instance: &VehicleRoutingProblem, client: ClientId) {
        let mut best_route_index = 0;
        let mut route_position = 0;
        let mut best_route_cost = f64::INFINITY;

        for route_idx in 0..self.routes.len() {
            let (best_index, best_cost) = self.lowest_cost_position(instance, client, route_idx);
            if best_cost < best_route_cost {
                best_route_index = route_idx;
                best_route_cost = best_cost;
                route_position = best_index;
            }
        }

        self.routes[best_route_index].insert(route_position, client);
    }

    fn insert_distance(&mut self, instance: &VehicleRoutingProblem, client: ClientId) {
        let mut best_route_index = 0;
        let mut best_route_proximity = f64::INFINITY;

        for route_idx in 0..self.routes.len() {
            let proximity =
                instance
                    .graph
                    .proximity(client, &self.routes[route_idx], self.routes.len());
            if proximity < best_route_proximity {
                best_route_index = route_idx;
                best_route_proximity = proximity;
            }
        }

        let (best_index, _) = self.lowest_cost_position(instance, client, best_route_index);
        self.routes[best_route_index].insert(best_index, client);
    }

    pub fn remove(
        &mut self,
        instance: &VehicleRoutingProblem,
        client: ClientId,
        size: usize,
        method: RemovalHeuristic,
    ) {
        match method {
            RemovalHeuristic::Concentric => self.remove_concentric(instance, client, size),
            RemovalHeuristic::Sequential => self.remove_sequential(instance, client, size),
        }
    }

    fn remove_concentric(
        &mut self,
        instance: &VehicleRoutingProblem,
        client: ClientId,
        size: usize,
    ) {
        let clients_to_remove = instance
            .graph
            .neighbors(client)
            .take(size - 1)
            .chain([client]);

        for client in clients_to_remove {
            self.remove(instance, client, 1, RemovalHeuristic::Sequential);
        }
    }

    fn remove_sequential(
        &mut self,
        _instance: &VehicleRoutingProblem,
        client: ClientId,
        size: usize,
    ) {
        let route_idx = self
            .routes
            .iter()
            .position(|route| route.contains(&client))
            .unwrap();

        let route = &mut self.routes[route_idx];
        let client_idx = route.iter().position(|c| c == &client).unwrap();

        for _ in 0..min(size, route.len() - client_idx) {
            route.remove(client_idx);
        }
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

    pub fn inter_shift(&mut self, instance: &VehicleRoutingProblem, client: ClientId) {
        let route_idx = self
            .routes
            .iter()
            .position(|route| route.contains(&client))
            .unwrap();
        let random_route = (0..self.routes.len())
            .filter(|&i| i != route_idx)
            .choose(&mut rand::thread_rng())
            .unwrap();

        let client_demand = instance.demand(client);
        let random_route_demand = self.routes[random_route]
            .iter()
            .map(|c| instance.demand(*c))
            .sum::<u32>();

        if (random_route_demand + client_demand) > instance.vehicle_capacity {
            return;
        }

        let (best_index, best_cost) = self.lowest_cost_position(instance, client, random_route);
        self.routes[random_route].insert(best_index, client);
    }

    pub fn inter_swap(
        &mut self,
        instance: &VehicleRoutingProblem,
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

                let (best_index, _) = self.lowest_cost_position(instance, client1, client2_route);
                self.routes[client2_route].insert(best_index, client1);

                let (best_index, _) = self.lowest_cost_position(instance, client2, client1_route);
                self.routes[client1_route].insert(best_index, client2);

                Ok(())
            }
            _ => Err(LocalSearchError::InvalidTour),
        }
    }

    pub fn intra_shift(&mut self, instance: &VehicleRoutingProblem, client: ClientId) {
        let route_idx = self
            .routes
            .iter()
            .position(|route| route.contains(&client))
            .unwrap();

        let client_idx = self.routes[route_idx]
            .iter()
            .position(|c| c == &client)
            .unwrap();

        let (best_index, _) = self.lowest_cost_position(instance, client, route_idx);
        self.routes[route_idx].remove(client_idx);
        self.routes[route_idx].insert(best_index, client);
    }

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

    fn lowest_cost_position(
        &self,
        instance: &VehicleRoutingProblem,
        client: ClientId,
        route_idx: usize,
    ) -> (usize, f64) {
        let mut best_index = 0;
        let mut best_cost = f64::INFINITY;

        let route = &self.routes[route_idx];

        for (i, next_client) in route.iter().chain([0].iter()).enumerate() {
            let prev_client = if i == 0 { 0 } else { route[i - 1] };

            let cost = instance.graph.distance(client, prev_client)
                + instance.graph.distance(client, *next_client)
                - instance.graph.distance(prev_client, *next_client);

            if cost < best_cost {
                best_index = i;
                best_cost = cost;
            }
        }

        (best_index, best_cost)
    }
}
