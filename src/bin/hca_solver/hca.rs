use core::time;

#[derive(Debug)]
pub struct HealthcareAnalyticsProblem {
    pub instance_name: String,
    pub number_of_tests: usize,
    pub number_of_diseases: usize,
    pub costs: Vec<f64>,
    pub distinguish_matrix: Vec<Vec<Vec<u8>>>,
}

#[derive(Debug)]
pub struct HealthcareAnalyticsSolution {
    pub instance_name: String,
    pub compute_time: time::Duration,
    pub is_optimal: bool,
    pub cost: f64,
}

impl HealthcareAnalyticsProblem {
    pub fn new(
        filename: &str,
        number_of_tests: usize,
        number_of_diseases: usize,
        costs: Vec<f64>,
        test_matrix: Vec<Vec<f64>>,
    ) -> Self {
        let distinguish_matrix = Vec::new();

        for test in 0..number_of_tests {
            let mut distinguishes = vec![vec![0; number_of_diseases]; number_of_diseases];
            for disease_i in 0..number_of_diseases {
                for disease_j in 0..number_of_diseases {
                    if test_matrix[test][disease_i] != test_matrix[test][disease_j] {
                        distinguishes[disease_i][disease_j] = 1;
                    }
                }
            }
        }

        HealthcareAnalyticsProblem {
            instance_name: filename.to_string(),
            number_of_tests,
            number_of_diseases,
            costs,
            distinguish_matrix,
        }
    }

    pub fn from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(filename)?;
        let mut number_of_tests = 0;
        let mut number_of_diseases = 0;
        let mut costs = Vec::new();
        let mut test_matrix = Vec::new();

        for (i, line) in contents.split("\n").enumerate() {
            if i == 0 {
                number_of_tests = line.trim().parse::<usize>()?;
            } else if i == 1 {
                number_of_diseases = line.trim().parse::<usize>()?;
            } else if i == 2 {
                costs = line
                    .split(" ")
                    .map(|x| x.parse::<f64>())
                    .collect::<Result<Vec<f64>, _>>()?;
            } else {
                let row: Vec<f64> = line
                    .split(" ")
                    .map(|x| x.parse::<f64>())
                    .collect::<Result<Vec<f64>, _>>()?;
                test_matrix.push(row);
            }
        }

        Ok(HealthcareAnalyticsProblem::new(
            filename,
            number_of_tests,
            number_of_diseases,
            costs,
            test_matrix,
        ))
    }

    pub fn solve(&self, timeout: Option<time::Duration>) -> HealthcareAnalyticsSolution {
        unimplemented!("Solve method not implemented yet");
    }
}

impl std::fmt::Display for HealthcareAnalyticsSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{{\"Instance\": {}, \"Time\": {:2}, \"Result\": {}, \"Solution\": {}}}",
            self.instance_name,
            self.compute_time.as_secs_f64(),
            self.is_optimal,
            self.cost,
        )
    }
}
