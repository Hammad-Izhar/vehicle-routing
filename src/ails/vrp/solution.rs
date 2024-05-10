use std::{io::Write, time::Duration};

#[derive(Debug)]
pub struct VehicleRoutingSolution {
    pub instance_name: String,
    pub compute_time: Duration,
    pub is_optimal: bool,
    pub cost: f64,
    pub routes: Vec<Vec<usize>>,
}

impl std::fmt::Display for VehicleRoutingSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{{\"Instance\": {}, \"Time\": {:2}, \"Result\": {}, \"Solution\": {}}}",
            self.instance_name,
            self.compute_time.as_secs_f64(),
            self.cost,
            self.routes
                .iter()
                .map(|route| format!(
                    "{}",
                    route
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                ))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl VehicleRoutingSolution {
    pub fn to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut output = std::fs::File::create(filename)?;

        output
            .write(format!("{} {}\n", self.cost, if self.is_optimal { 1 } else { 0 }).as_bytes())?;
        for route in &self.routes {
            output.write(
                format!(
                    "{}\n",
                    route
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
                .as_bytes(),
            )?;
        }

        Ok(())
    }
}
