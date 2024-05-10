use crate::ails::vrp::problem::VehicleRoutingProblem;

pub enum AILSPhase {
    PhaseOne,
    PhaseTwo,
}

pub enum RemovalHeuristic {
    Concentric,
    Sequential,
}

pub enum InsertionHeuristic {
    Cost,
    Distance,
}

struct AILS {
    phase: AILSPhase,
    instance: VehicleRoutingProblem,
}

impl AILS {
    pub fn new(instance: VehicleRoutingProblem) -> Self {
        Self {
            phase: AILSPhase::PhaseOne,
            instance,
        }
    }
}
