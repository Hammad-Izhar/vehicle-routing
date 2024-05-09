use std::{cmp::Reverse, collections::HashSet};

use priority_queue::priority_queue::PriorityQueue;

use crate::unordered_pair::UnorderedPair;
use crate::vrp::Client;

#[derive(Debug)]
pub struct VehicleRoutingGraph {
    pub clients: Vec<Client>,
    pub distance_matrix: Vec<Vec<ordered_float::OrderedFloat<f64>>>,
}

impl VehicleRoutingGraph {
    pub fn new(clients: &[Client]) -> Self {
        let mut distance_matrix = vec![vec![0.0.into(); clients.len()]; clients.len()];
        for client in clients.iter() {
            for other_client in clients.iter() {
                let distance = ((client.x - other_client.x).powi(2)
                    + (client.y - other_client.y).powi(2))
                .sqrt();
                distance_matrix[client.id][other_client.id] = distance.into();
            }
        }

        VehicleRoutingGraph {
            clients: clients.to_vec(),
            distance_matrix,
        }
    }

    /// Computes the minimimum spanning tree of the graph using Prim's algorithm
    ///
    /// Prim's Algorithm:
    ///   1. Start with an arbitrary vertex as the root
    ///   2. Add the edge with the smallest weight to the tree
    ///   3. Add the edge with the smallest weight that connects a vertex in the tree to a vertex outside the tree
    ///   4. Repeat step 3 until all vertices are in the tree
    ///
    /// Panics (in debug mode) if:
    ///     - If the graph is not connected
    ///
    /// Future Work:
    ///     - For Euclidean MST, we could compute the MST using the Delaunay triangulation
    ///
    /// Returns a list of edges a.k.a UnorderedPair<Client> that form the MST
    pub fn mst(&self) -> Vec<UnorderedPair<&Client>> {
        let mut included_clients = HashSet::new();
        let mut tree = Vec::new();
        let mut pq = PriorityQueue::new();

        let root = &self.clients[0];

        // Select the first client as the root
        included_clients.insert(root);

        // Add all edges from the root to the other clients
        for other_client in self.clients.iter().skip(1) {
            pq.push(
                UnorderedPair::new(root, other_client),
                Reverse(self.distance_matrix[root.id][other_client.id]),
            );
        }

        while included_clients.len() < self.clients.len() {
            // Choose the edge with the smallest weights
            let (edge, _) = pq.pop().unwrap();
            let UnorderedPair { first, second } = edge;

            // Determine the new fringe client that we are visiting, if it exists
            let new_client = if !included_clients.contains(&first) {
                first
            } else if !included_clients.contains(&second) {
                second
            } else {
                continue;
            };

            // Add the new client and edge to the tree
            included_clients.insert(new_client);
            tree.push(edge);

            // Add all edges from the new client to the other clients
            for client in self.clients.iter() {
                if included_clients.contains(client) {
                    continue;
                }
                pq.push(
                    UnorderedPair::new(new_client, client),
                    Reverse(self.distance_matrix[new_client.id][client.id]),
                );
            }
        }

        // Ensure that all clients are included in the tree
        assert!(included_clients.len() == self.clients.len());
        // Ensure that the tree has the correct number of edges
        assert!(tree.len() == self.clients.len() - 1);
        tree
    }

    pub fn find_minimum_weight_matching(&self, subset: &[Client]) -> Vec<UnorderedPair<&Client>> {
        unimplemented!("Coming soon!");
    }

    pub fn find_eulerian_tour(
        &self,
        mst: &[UnorderedPair<&Client>],
        matching: &[UnorderedPair<&Client>],
    ) -> Vec<&Client> {
        unimplemented!("Coming soon!");
    }

    pub fn convert_eulerian_tour_to_tsp(&self, eulerian_tour: &[&Client]) -> Vec<&Client> {
        unimplemented!("Coming soon!");
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
        assert!(mst.contains(&UnorderedPair::new(&clients[0], &clients[2])));
        assert!(mst.contains(&UnorderedPair::new(&clients[0], &clients[4])));
        assert!(mst.contains(&UnorderedPair::new(&clients[0], &clients[5])));
        assert!(mst.contains(&UnorderedPair::new(&clients[1], &clients[2])));
        assert!(mst.contains(&UnorderedPair::new(&clients[3], &clients[4])));
    }
}
