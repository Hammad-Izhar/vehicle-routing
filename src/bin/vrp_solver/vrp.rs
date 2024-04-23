use std::{io::Write, time::Duration};

use crate::graph::VehicleRoutingGraph;
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Hash, Clone)]
pub struct Client {
    pub id: usize,
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    pub demand: u32,
}

#[derive(Debug)]
pub struct VehicleRoutingProblem {
    pub instance_name: String,
    pub number_of_customers: usize,
    pub number_of_vehicles: usize,
    pub vehicle_capacity: u32,
    pub graph: VehicleRoutingGraph,
}
#[derive(Debug)]
pub struct VehicleRoutingSolution {
    pub instance_name: String,
    pub compute_time: Duration,
    pub is_optimal: bool,
    pub cost: f64,
    pub routes: Vec<Vec<usize>>,
}

impl Client {
    fn new(id: usize, x: OrderedFloat<f64>, y: OrderedFloat<f64>, demand: u32) -> Self {
        Client { id, x, y, demand }
    }
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
                number_of_customers = parts[0].parse::<usize>()?;
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

    pub fn solve(&self, timeout: Option<Duration>) -> VehicleRoutingSolution {
        unimplemented!("Coming soon!");
    }
}

impl std::fmt::Display for VehicleRoutingSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{{\"Instance\": {}, \"Time\": {:2}, \"Result\": {}, \"Solution\": {}}}",
            self.instance_name,
            self.compute_time.as_secs_f64(),
            self.cost,
            self.routes
                .iter()
                .map(|route| format!(
                    "{}",
                    route
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl VehicleRoutingSolution {
    pub fn to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut output = std::fs::File::create(filename)?;

        output
            .write(format!("{} {}\n", self.cost, if self.is_optimal { 1 } else { 0 }).as_bytes())?;
        for route in &self.routes {
            output.write(
                format!(
                    "{}\n",
                    route
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
                .as_bytes(),
            )?;
        }

        Ok(())
    }
}
