use std::{cmp::min, collections::HashMap};

use rand::Rng;

use crate::ails::vrp::client::Client;

use super::client::ClientId;
#[derive(Debug)]
pub struct VehicleRoutingGraph {
    pub clients: Vec<Client>,
    distance_matrix: Vec<Vec<f64>>,
    closest_clients: HashMap<ClientId, Vec<ClientId>>,
}

impl VehicleRoutingGraph {
    pub fn new(clients: &[Client]) -> Self {
        let mut distance_matrix = vec![vec![0.0; clients.len()]; clients.len()];
        for client in clients {
            for other_client in clients {
                let distance = ((client.x - other_client.x).powi(2)
                    + (client.y - other_client.y).powi(2))
                .sqrt();
                distance_matrix[client.id][other_client.id] = distance;
            }
        }

        let mut closest_clients = HashMap::new();
        for i in 0..clients.len() {
            let mut distances = (1..clients.len())
                .map(|j| (j, distance_matrix[i][j]))
                .collect::<Vec<(usize, f64)>>();
            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let sorted_neighbors = distances.iter().map(|(j, _)| *j).collect();
            closest_clients.insert(i, sorted_neighbors);
        }

        VehicleRoutingGraph {
            clients: clients.to_vec(),
            distance_matrix,
            closest_clients,
        }
    }

    pub fn distance(&self, client1: usize, client2: usize) -> f64 {
        self.distance_matrix[client1][client2]
    }

    pub fn distance_to_depot(&self, client: usize) -> f64 {
        self.distance(0, client)
    }

    pub fn neighbors(&self, client: ClientId) -> impl Iterator<Item = ClientId> + '_ {
        self.closest_clients[&client].iter().map(|&n| n)
    }

    pub fn proximity(&self, client: ClientId, route: &[ClientId], num_routes: usize) -> f64 {
        let proximity_set = route.iter().map(|&c| {
            self.closest_clients[&client]
                .iter()
                .position(|&n| n == c)
                .unwrap() as f64
        });

        let rho = min(
            route.len() - 2,
            rand::thread_rng().gen_range(1..=self.clients.len() / num_routes),
        );

        proximity_set.take(rho).sum::<f64>() / rho as f64
    }
}
