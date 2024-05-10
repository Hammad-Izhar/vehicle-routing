use std::{io::Write, time::Duration};

use crate::graph::VehicleRoutingGraph;
use log::{info, trace};
use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    pub fn new(id: usize, x: OrderedFloat<f64>, y: OrderedFloat<f64>, demand: u32) -> Self {
        Client { id, x, y, demand }
    }
}

impl PartialOrd for Client {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Client {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
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

    fn christofides(&self) -> (Vec<Client>, f64) {
        // Step 1: Compute the minimum spanning tree of the graph
        let start_time = std::time::Instant::now();
        let mst = self.graph.mst();

        // Step 2: Find the set of vertices with odd degree in the minimum spanning tree
        let mut client_degrees = vec![0; self.number_of_customers];
        for edge in &mst {
            client_degrees[edge.first.id] += 1;
            client_degrees[edge.second.id] += 1;
        }

        let odd_degree_clients = client_degrees
            .iter()
            .enumerate()
            .filter(|(_, d)| **d % 2 == 1)
            .map(|(client_id, _)| self.graph.clients[client_id].clone())
            .collect::<Vec<Client>>();

        // By the handshaking lemma, the number of vertices with odd degree must be even
        assert!(odd_degree_clients.len() % 2 == 0);
        info!("Computing MST took: {:?}", start_time.elapsed());

        trace!(
            "A total of {} clients have odd degree in the MST",
            odd_degree_clients.len()
        );

        // Step 3: Compute the minimum weight perfect matching of the odd degree vertices
        let start_time = std::time::Instant::now();
        let matching = self.graph.find_minimum_weight_matching(&odd_degree_clients);
        info!("Blossom Algorithm took: {:?}", start_time.elapsed());

        // Step 4: Add the minimum weight perfect matching to the minimum spanning tree
        // Step 5: Find an Eulerian tour of the graph
        let start_time = std::time::Instant::now();
        let eulerian_tour = VehicleRoutingGraph::find_eulerian_tour(&mst, &matching);
        info!("Computing Eulerian Tour took: {:?}", start_time.elapsed());

        // Step 6: Convert the Eulerian tour into a TSP by skipping repeated vertices
        let start_time = std::time::Instant::now();
        let (tsp, tsp_weight) = self.graph.convert_eulerian_tour_to_tsp(&eulerian_tour);
        info!("Shortcutting took: {:?}", start_time.elapsed());

        return (tsp, tsp_weight.into());
    }

    fn partition_tour(&self, tour: Vec<Client>) -> Option<(Vec<Vec<Client>>, f64)> {
        let mut best_partition = None;

        let depot = &tour[0];
        let mut tour_without_depot = tour[1..tour.len() - 1].to_vec();

        for _ in 0..tour_without_depot.len() {
            let mut current_partition = vec![vec![]; self.number_of_vehicles];
            let mut partition_demands = vec![0; self.number_of_vehicles];

            for client in &tour_without_depot {
                for (vehicle_route, demand) in current_partition
                    .iter_mut()
                    .zip(partition_demands.iter_mut())
                {
                    if *demand + client.demand <= self.vehicle_capacity {
                        vehicle_route.push(*client);
                        *demand += client.demand;
                        break;
                    }
                }
            }

            let clients_visited = current_partition
                .iter()
                .flatten()
                .collect::<Vec<&Client>>()
                .len();

            if clients_visited != self.number_of_customers - 1 {
                continue;
            }

            for route in current_partition.iter_mut() {
                route.insert(0, *depot);
                route.push(*depot);
            }

            let current_partition_cost = current_partition
                .iter()
                .map(|route| {
                    route
                        .windows(2)
                        .map(|pair| self.graph.distance_matrix[pair[0].id][pair[1].id])
                        .sum::<OrderedFloat<f64>>()
                })
                .sum::<OrderedFloat<f64>>();

            best_partition = match best_partition {
                Some((_, best_cost)) => {
                    if current_partition_cost < best_cost {
                        Some((current_partition.clone(), current_partition_cost))
                    } else {
                        best_partition
                    }
                }
                None => Some((current_partition.clone(), current_partition_cost)),
            };

            tour_without_depot.rotate_left(1);
        }

        return best_partition.map(|(partition, cost)| (partition, cost.into()));
    }

    /// Use the Christofides algorithm to find an inital feasible solution
    /// Followed by a local search/branch-and-bound algorithm to improve the solution
    ///
    /// Chistofides Algorithm:
    ///    1. Compute the minimum spanning tree of the graph
    ///    2. Find the set of vertices with odd degree in the minimum spanning tree
    ///    3. Compute the minimum weight perfect matching of the odd degree vertices
    ///    4. Add the minimum weight perfect matching to the minimum spanning tree
    ///    5. Find an Eulerian tour of the graph
    ///    6. Convert the Eulerian tour into a TSP by skipping repeated vertices
    ///
    /// LP Formulation:
    ///     TBD!
    pub fn solve(&self, timeout: Option<Duration>) -> VehicleRoutingSolution {
        let start_time = std::time::Instant::now();
        let (tsp, tsp_weight) = self.christofides();
        info!("Christofides took: {:?}", start_time.elapsed());

        let start_time = std::time::Instant::now();
        let (best_partition, best_partition_cost) =
            self.partition_tour(tsp).map_or((vec![], 0.0), |x| x);
        info!("Partitioning took: {:?}", start_time.elapsed());

        // TODO: Implement a local search algorithm or branch and bound to improve the solution

        VehicleRoutingSolution {
            instance_name: self.instance_name.clone(),
            compute_time: timeout.unwrap_or(Duration::from_secs(0)),
            is_optimal: false,
            cost: best_partition_cost,
            routes: best_partition
                .iter()
                .map(|route| route.iter().map(|c| c.id).collect())
                .collect(),
        }
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
