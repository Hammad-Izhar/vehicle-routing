use log::info;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

pub struct Incumbent<T> {
    pub solution: T,
    pub cost: f64,
}

pub fn branch_and_bound<T>(initial_incumbent: Incumbent<T>) -> Incumbent<T> {
    // If no initial incumbent is provided, then find one using DFS
    let mut incumbent = initial_incumbent;

    // NOTE: for the purposes of the VRP solver, the initial incumbent will typically be found using an approimation algorithm.

    // Initialize a priority queue to store the nodes to be explored (best-first order)
    let mut pq = PriorityQueue::new();

    while !pq.is_empty() {
        let (node_configuration, parent_cost) = pq.pop().unwrap();

        if incumbent.cost < parent_cost {
            info!(
                "Pruning node with cost {} since the current incumbent has cost {}",
                parent_cost, incumbent.cost
            );
            continue;
        }

        // build model, compute model solution

        if ... {
        // if model is feasible
            if ... {
                // if model solution is an integer solution
                if ... {
                    // if model solution is better than incumbent
                    if ... {
                        incumbent = Incumbent {
                            solution: ...,
                            cost: ...,
                        };
                    }
                } else {
                    // if model solution is fractional
                    // branch on the fractional variable
                    // add the two nodes to the priority queue
                }
            }
        }
    }

    // While there are more nodes to be explored
    // Perform branch and bound :D

    incumbent
}
