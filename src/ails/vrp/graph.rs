use std::{
    cmp::{min, Reverse},
    collections::{HashMap, HashSet},
};

use blossom::graph::AnnotatedGraph;
use log::{info, trace};
use ordered_float::OrderedFloat;
use priority_queue::PriorityQueue;
use rand::Rng;

use crate::ails::vrp::client::Client;

use super::{client::ClientId, unordered_pair::UnorderedPair};
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

    fn mst(&self) -> Vec<UnorderedPair<ClientId>> {
        let mut included_clients = HashSet::new();
        let mut tree = Vec::new();
        let mut pq: PriorityQueue<UnorderedPair<usize>, Reverse<OrderedFloat<f64>>> =
            PriorityQueue::new();

        let root = 0;

        included_clients.insert(root);

        for other_client in 1..self.clients.len() {
            pq.push(
                UnorderedPair::new(root, other_client),
                Reverse(self.distance(root, other_client).into()),
            );
        }

        while included_clients.len() < self.clients.len() {
            let (edge, _) = pq.pop().unwrap();
            let UnorderedPair { first, second } = edge;

            let new_client = if !included_clients.contains(&first) {
                first
            } else if !included_clients.contains(&second) {
                second
            } else {
                continue;
            };

            included_clients.insert(new_client);
            tree.push(edge);

            for client in 0..self.clients.len() {
                if included_clients.contains(&client) {
                    continue;
                }

                pq.push(
                    UnorderedPair::new(new_client, client),
                    Reverse(self.distance(new_client, client).into()),
                );
            }
        }

        assert!(included_clients.len() == self.clients.len());
        // Ensure that the tree has the correct number of edges
        assert!(tree.len() == self.clients.len() - 1);
        tree
    }

    pub fn find_minimum_weight_matching(
        &self,
        subset: &[ClientId],
    ) -> Vec<UnorderedPair<ClientId>> {
        let mut edges = HashMap::new();

        for client in subset.iter() {
            let incident_clients = subset
                .iter()
                .filter(|c| *c != client)
                .map(|c| *c)
                .collect::<Vec<usize>>();

            let incident_weights = subset
                .iter()
                .filter(|c| *c != client)
                .map(|c| self.distance(*client, *c))
                .collect::<Vec<f64>>();
            edges.insert(*client, (incident_clients, incident_weights));
        }

        let graph = AnnotatedGraph::new(edges);

        let matching = graph
            .maximin_matching()
            .expect("Unable to find perfect matching");

        matching
            .edges()
            .iter()
            .map(|(u, v)| UnorderedPair::new(*u, *v))
            .collect::<Vec<UnorderedPair<usize>>>()
    }

    pub fn find_eulerian_tour(
        mst: &[UnorderedPair<ClientId>],
        matching: &[UnorderedPair<ClientId>],
    ) -> Vec<ClientId> {
        let mut vertex_to_edges = HashMap::new();
        for edge in mst.iter().chain(matching.iter()) {
            vertex_to_edges
                .entry(edge.first)
                .or_insert_with(HashSet::new)
                .insert(*edge);
            vertex_to_edges
                .entry(edge.second)
                .or_insert_with(HashSet::new)
                .insert(*edge);
        }

        let mut eulerian_tour = Vec::new();
        let mut stack = vec![mst[0].first];

        while !stack.is_empty() {
            let current_vertex = stack.last().unwrap();
            let edges = vertex_to_edges.get_mut(current_vertex).unwrap();

            if edges.is_empty() {
                eulerian_tour.push(stack.pop().unwrap());
            } else {
                let edge = *edges.iter().next().unwrap();
                let next_vertex = if edge.first == *current_vertex {
                    edge.second
                } else {
                    edge.first
                };

                edges.remove(&edge);
                vertex_to_edges.get_mut(&next_vertex).unwrap().remove(&edge);
                stack.push(next_vertex);
            }
        }

        let depot_index = eulerian_tour.iter().position(|&c| c == 0).unwrap();
        eulerian_tour.rotate_left(depot_index);

        eulerian_tour
    }

    fn convert_eulerian_tour_to_circuit(eulerian_tour: &[ClientId]) -> Vec<ClientId> {
        let mut visited = HashSet::new();
        let mut circuit = Vec::new();

        for client in eulerian_tour {
            if !visited.contains(client) {
                circuit.push(*client);
                visited.insert(client);
            }
        }

        circuit.push(0);
        circuit
    }

    pub fn chirstofides(&self) -> Vec<ClientId> {
        // Step 1: Compute the minimum spanning tree of the graph
        let start_time = std::time::Instant::now();
        let mst = self.mst();

        let mut client_degrees = vec![0; self.clients.len()];
        for edge in mst.iter() {
            client_degrees[edge.first] += 1;
            client_degrees[edge.second] += 1;
        }

        let odd_clients = (0..self.clients.len())
            .filter(|&client| client_degrees[client] % 2 == 1)
            .collect::<Vec<usize>>();

        assert!(odd_clients.len() % 2 == 0);
        trace!("Computed MST in {:?}", start_time.elapsed());

        // Step 2: Find a minimum weight perfect matching of the odd degree vertices
        let start_time = std::time::Instant::now();
        let matching = self.find_minimum_weight_matching(&odd_clients);
        trace!(
            "Computed minimum weight matching in {:?}",
            start_time.elapsed()
        );

        // Step 3: Combine the minimum spanning tree and the matching to form a multigraph
        // Step 4: Find an Eulerian tour of the multigraph
        let start_time = std::time::Instant::now();
        let eulerian_tour = VehicleRoutingGraph::find_eulerian_tour(&mst, &matching);
        trace!("Computed Eulerian tour in {:?}", start_time.elapsed());

        // Step 5: Convert the Eulerian tour into a Hamiltonian circuit
        let start_time = std::time::Instant::now();
        let circuit = VehicleRoutingGraph::convert_eulerian_tour_to_circuit(&eulerian_tour);
        trace!(
            "Converted Eulerian tour to Hamiltonian circuit in {:?}",
            start_time.elapsed()
        );

        circuit
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_basic_mst() {
        let clients = vec![
            Client::new(0, 0.0.into(), 0.0.into(), 0),
            Client::new(1, 0.0.into(), 7.0.into(), 0),
            Client::new(2, 3.0.into(), 4.0.into(), 0),
            Client::new(3, 7.0.into(), (-10.0).into(), 0),
            Client::new(4, (-4.0).into(), (-6.0).into(), 0),
            Client::new(5, (-4.0).into(), 3.0.into(), 0),
        ];

        let graph = VehicleRoutingGraph::new(&clients);

        let mst = graph.mst();
        assert_eq!(mst.len(), 5);
        assert!(mst.contains(&UnorderedPair::new(0, 2)));
        assert!(mst.contains(&UnorderedPair::new(0, 4)));
        assert!(mst.contains(&UnorderedPair::new(0, 5)));
        assert!(mst.contains(&UnorderedPair::new(1, 2)));
        assert!(mst.contains(&UnorderedPair::new(3, 4)));
    }

    #[test]
    fn test_eulerian_tour() {
        let clients = vec![
            Client::new(0, 0.0.into(), 0.0.into(), 0),
            Client::new(1, 0.0.into(), 7.0.into(), 0),
            Client::new(2, 3.0.into(), 4.0.into(), 0),
            Client::new(3, 7.0.into(), (-10.0).into(), 0),
            Client::new(4, (-4.0).into(), (-6.0).into(), 0),
            Client::new(5, (-4.0).into(), 3.0.into(), 0),
        ];

        let mst = vec![
            UnorderedPair::new(0, 2),
            UnorderedPair::new(0, 4),
            UnorderedPair::new(1, 2),
            UnorderedPair::new(1, 5),
            UnorderedPair::new(3, 4),
            UnorderedPair::new(3, 5),
        ];

        let tour = VehicleRoutingGraph::find_eulerian_tour(&mst, &[]);

        assert_eq!(tour.len(), mst.len() + 1);
        assert_eq!(*tour.first().unwrap(), 0);
        let mut current_vertex = tour.first().unwrap();
        for next_vertex in tour.iter().skip(1) {
            assert!(mst.contains(&UnorderedPair::new(*current_vertex, *next_vertex)));
            current_vertex = next_vertex;
        }
    }
}
