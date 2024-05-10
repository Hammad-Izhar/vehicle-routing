use std::collections::HashMap;

use crate::ails::vrp::client::Client;

use super::client::ClientId;
#[derive(Debug)]
pub struct VehicleRoutingGraph {
    pub clients: Vec<Client>,
    distance_matrix: Vec<Vec<f64>>,
    closest_clients: HashMap<ClientId, Vec<ClientId>>,
}

impl VehicleRoutingGraph {
    pub fn new(clients: &[Client], k_nearest: usize) -> Self {
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
            let mut distances = (0..clients.len())
                .map(|j| (j, distance_matrix[i][j]))
                .collect::<Vec<(usize, f64)>>();
            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let nearest_neighbors = distances.iter().take(k_nearest).map(|(j, _)| *j).collect();
            closest_clients.insert(i, nearest_neighbors);
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

    pub fn neighbors(&self, client: ClientId) -> Vec<ClientId> {
        self.closest_clients[&client].clone()
    }
}
