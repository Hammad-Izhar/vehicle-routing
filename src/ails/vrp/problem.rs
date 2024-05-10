use std::time::Duration;

use ordered_float::OrderedFloat;

use crate::ails::vrp::client::{Client, ClientId};
use crate::ails::vrp::graph::VehicleRoutingGraph;

use crate::ails::vrp::solution::VehicleRoutingSolution;

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
        unimplemented!()
    }
}
