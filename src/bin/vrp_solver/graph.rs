use crate::vrp::Client;

#[derive(Debug)]
pub struct VehicleRoutingGraph {
    clients: Vec<Client>,
}

impl VehicleRoutingGraph {
    pub fn new(clients: &[Client]) -> Self {
        VehicleRoutingGraph {
            clients: clients.to_vec(),
        }
    }
}
