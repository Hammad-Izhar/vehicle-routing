use std::time::Duration;

use log::info;
use ordered_float::OrderedFloat;

use crate::ails::ails::AILS;
use crate::ails::vrp::client::{Client, ClientId};
use crate::ails::vrp::graph::VehicleRoutingGraph;

use crate::ails::vrp::solution::VehicleRoutingSolution;

use super::routing_plan::RoutingPlan;

#[derive(Debug)]
pub struct VehicleRoutingProblem {
    pub instance_name: String,
    pub number_of_customers: usize,
    pub number_of_vehicles: usize,
    pub vehicle_capacity: u32,
    pub graph: VehicleRoutingGraph,
}

impl VehicleRoutingProblem {
    pub fn from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file_contents = std::fs::read_to_string(filename)?;
        let mut number_of_customers = 0;
        let mut number_of_vehicles = 0;
        let mut vehicle_capacity = 0;
        let mut clients = Vec::new();

        for (i, line) in file_contents.split("\n").enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if i == 0 {
                let parts: Vec<&str> = line.split(" ").collect();
                number_of_customers = parts[0].parse::<usize>()? - 1; // don't include the depot
                number_of_vehicles = parts[1].parse::<usize>()?;
                vehicle_capacity = parts[2].parse::<u32>()?;
            } else {
                let parts: Vec<&str> = line.split(" ").collect();
                let demand = parts[0].parse::<u32>()?;
                let x = parts[1].parse::<OrderedFloat<f64>>()?;
                let y = parts[2].parse::<OrderedFloat<f64>>()?;

                clients.push(Client::new(i - 1, x, y, demand))
            }
        }

        Ok(VehicleRoutingProblem {
            instance_name: filename.to_string(),
            number_of_customers,
            number_of_vehicles,
            vehicle_capacity,
            graph: VehicleRoutingGraph::new(&clients),
        })
    }

    pub fn demand(&self, client: ClientId) -> u32 {
        self.graph.clients[client].demand
    }

    pub fn solve(&self, timeout: Option<Duration>) -> VehicleRoutingSolution {
        let start_time = std::time::Instant::now();
        let tsp = self.graph.chirstofides();
        let initial_routing_plan = self.partition_tour(tsp);

        let mut ails = AILS::new();

        let solution = ails.run(self, initial_routing_plan, std::time::Instant::now());

        VehicleRoutingSolution {
            instance_name: self.instance_name.clone(),
            compute_time: start_time.elapsed(),
            is_optimal: false,
            cost: solution.value(self),
            routes: solution.routes,
        }
    }

    fn partition_tour(&self, tour: Vec<ClientId>) -> Option<RoutingPlan> {
        let mut best_partition = None;
        let mut tour_without_depot = tour[1..tour.len() - 1].to_vec();

        for _ in 0..tour_without_depot.len() {
            let mut current_partition = vec![vec![]; self.number_of_vehicles];
            let mut partition_demands = vec![0; self.number_of_vehicles];

            for client in &tour_without_depot {
                for (vehicle_route, demand) in current_partition
                    .iter_mut()
                    .zip(partition_demands.iter_mut())
                {
                    if *demand + self.demand(*client) <= self.vehicle_capacity {
                        vehicle_route.push(*client);
                        *demand += self.demand(*client);
                        break;
                    }
                }
            }

            let partition = RoutingPlan::new(current_partition);
            if let Err(_) = partition.feasible(self) {
                continue;
            }

            best_partition = match best_partition {
                None => Some(partition),
                Some(best) => {
                    if partition.value(self) < best.value(self) {
                        Some(partition)
                    } else {
                        Some(best)
                    }
                }
            };

            tour_without_depot.rotate_left(1);
        }

        best_partition
    }
}
