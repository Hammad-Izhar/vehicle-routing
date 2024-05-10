use ordered_float::OrderedFloat;

pub type ClientId = usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Client {
    pub id: ClientId,
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    pub demand: u32,
}

impl Client {
    pub fn new(id: ClientId, x: OrderedFloat<f64>, y: OrderedFloat<f64>, demand: u32) -> Self {
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
