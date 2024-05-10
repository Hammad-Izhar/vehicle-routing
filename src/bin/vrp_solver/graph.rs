use std::collections::HashMap;
use std::{cmp::Reverse, collections::HashSet};

use blossom::graph::AnnotatedGraph;
use clarabel::solver::DefaultProblemData;
use log::trace;
use ordered_float::OrderedFloat;
use priority_queue::priority_queue::PriorityQueue;

use crate::unordered_pair::UnorderedPair;
use crate::vrp::Client;

#[derive(Debug)]
pub struct VehicleRoutingGraph {
    pub clients: Vec<Client>,
    pub distance_matrix: Vec<Vec<OrderedFloat<f64>>>,
}

impl VehicleRoutingGraph {
    pub fn new(clients: &[Client]) -> Self {
        let mut distance_matrix = vec![vec![0.0.into(); clients.len()]; clients.len()];
        for client in clients {
            for other_client in clients {
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

    /// Computes the minimum weight matching of a subset of clients using the Blossom Algorithm
    ///
    /// Currently implemented using a package
    ///
    /// Future Work:
    ///    - Implement the Blossom Algorithm from scratch
    ///    - Implement Parallel/Distributed Blossom Algorithm
    pub fn find_minimum_weight_matching(&self, subset: &[Client]) -> Vec<UnorderedPair<&Client>> {
        trace!(
            "Finding minimum weight matching for {} clients",
            subset.len()
        );
        let mut edges = HashMap::new();
        for client in subset.iter() {
            let incident_edges: Vec<usize> = subset
                .iter()
                .filter(|c| *c != client)
                .map(|c| c.id)
                .collect::<Vec<usize>>();
            let incident_weights = subset
                .iter()
                .filter(|c| *c != client)
                .map(|c| self.distance_matrix[client.id][c.id].into())
                .collect::<Vec<f64>>();
            edges.insert(client.id, (incident_edges, incident_weights));
        }

        let graph = AnnotatedGraph::new(edges);

        let matching = graph
            .maximin_matching()
            .expect("Unable to find perfect matching");

        matching
            .edges()
            .iter()
            .map(|(u, v)| UnorderedPair::new(&self.clients[*u], &self.clients[*v]))
            .collect::<Vec<UnorderedPair<&Client>>>()
    }

    /// Computes an Eulerian tour of the multigraph formed by the minimum spanning tree and the minimum weight matching
    ///
    /// Heirholzer's Algorithm:
    ///    1. Start at any vertex
    ///    2. Follow a trail of edges from that vertex until returning to the vertex
    ///    3. It is not possible to get stuck at any vertex other than the starting vertex
    ///    4. If all edges have been visited, the tour is done
    ///    5. Otherwise, choose a vertex in the tour with unused edges and start a new trail from that vertex
    ///
    pub fn find_eulerian_tour<'a>(
        mst: &[UnorderedPair<&'a Client>],
        matching: &[UnorderedPair<&'a Client>],
    ) -> Vec<&'a Client> {
        let mut vertex_to_edges = HashMap::new();
        for edge in mst.iter().chain(matching.iter()) {
            vertex_to_edges
                .entry(edge.first)
                .or_insert_with(HashSet::new)
                .insert(edge);
            vertex_to_edges
                .entry(edge.second)
                .or_insert_with(HashSet::new)
                .insert(edge);
        }

        let mut eulerian_tour = Vec::new();
        let mut stack = vec![mst.first().unwrap().first];

        while !stack.is_empty() {
            let current_vertex = stack.last().unwrap();
            let edges = vertex_to_edges
                .get_mut(current_vertex)
                .expect("Unable to lookup vertex in edge hashmap");
            if edges.is_empty() {
                eulerian_tour.push(stack.pop().unwrap());
            } else {
                let edge = *edges.iter().next().unwrap();
                let next_vertex = if edge.first == *current_vertex {
                    edge.second
                } else {
                    edge.first
                };

                edges.remove(edge);
                vertex_to_edges.get_mut(&next_vertex).unwrap().remove(edge);
                stack.push(next_vertex);
            }
        }

        // Rotate the cycle so that the depot is the first and last vertex
        let depot_index = eulerian_tour
            .iter()
            .position(|client| client.id == 0)
            .expect("Unable to find the depot in the Eulerian tour");
        eulerian_tour.rotate_left(depot_index);

        eulerian_tour
    }

    pub fn convert_eulerian_tour_to_tsp<'a>(
        &self,
        eulerian_tour: &'a [&'a Client],
    ) -> (Vec<Client>, OrderedFloat<f64>) {
        let mut visited = HashSet::new();
        let mut tsp = Vec::new();
        let mut weight = 0.0.into();

        for client in eulerian_tour {
            if !visited.contains(*client) {
                tsp.push(**client);
                visited.insert(*client);
                if let Some(last_client) = tsp.last() {
                    weight += self.distance_matrix[last_client.id][client.id];
                }
            }
        }

        tsp.push(*eulerian_tour[0]);
        weight += self.distance_matrix[tsp.last().unwrap().id][tsp.first().unwrap().id];

        (tsp, weight)
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

        let mst: Vec<UnorderedPair<&Client>> = vec![
            UnorderedPair::new(&clients[0], &clients[2]),
            UnorderedPair::new(&clients[0], &clients[4]),
            UnorderedPair::new(&clients[1], &clients[2]),
            UnorderedPair::new(&clients[1], &clients[5]),
            UnorderedPair::new(&clients[3], &clients[4]),
            UnorderedPair::new(&clients[3], &clients[5]),
        ];

        let tour = VehicleRoutingGraph::find_eulerian_tour(&mst, &[]);

        println!("{:?}", tour);

        assert_eq!(tour.len(), mst.len() + 1);
        assert_eq!(tour.first().unwrap().id, 0);
        let mut current_vertex = tour.first().unwrap();
        for next_vertex in tour.iter().skip(1) {
            assert!(mst.contains(&UnorderedPair::new(current_vertex, next_vertex)));
            current_vertex = next_vertex;
        }
    }
}
