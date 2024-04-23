# Vehicle Routing

## About

> :exclamation: **Coming Soon!**

## Solution Method

> :exclamation: **Coming Soon!**

## Project Structure

All [https://github.com/Hammad-Izhar/vehicle-routing](code) is implemented in Rust and uses `cargo` to handle dependencies. Rather than using IBM's proprietary solver `CPLEX` I opted to use [https://github.com/oxfordcontrol/Clarabel.rs](`Clarabel.rs`), an "interior-point solver for convex conic optimization problems in Rust".

> :exclamation: **Remark**: `Clarabel.rs` does not support IPs, therefore, making the branch-and-bound implementation necessary!

Rather than interfacing with `Clarabel.rs` directly, `good_lp` is used. This allows the project to support a wide variety of solvers! For example, with a valid `CPLEX` installation/license, IBM's `CPLEX` is supported via [https://crates.io/crates/cplex-rs](`cplex-rs`).

```
> tree vehicle-routing
├── Cargo.lock
├── Cargo.toml
├── compile.sh
├── input
│   ├── ip/         # test instances for distributed branch-and bound solver
│   └── vrp/        # test instances for vrp solver
├── README.md       # this file!
├── runAll.sh
├── run.sh
├── src
│   └── bin
│       └── vrp_solver      # vrp solver executable
│           └── main.rs
└── team.txt
```

## Results

> :exclamation: **Coming Soon!**

## Future Work

> :exclamation: **Coming Soon!**

-   [ ] [https://arxiv.org/abs/1901.07032](Capacitated Vehicle Routing) implementation
-   [ ] Branch-and-Bound IP Solver Implementation
    -   [ ] Basic Implementation
    -   [ ] Multi-Threaded
    -   [ ] Distributed via MPI
-   [ ] ???
