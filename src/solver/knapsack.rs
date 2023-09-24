use rand::Rng;
use std::{collections::BTreeMap, fmt, fs};

use crate::config::Config;

use super::parser::{Product, Resource};
use super::Individual;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KnapSack {
    pub products: BTreeMap<String, Product>,
    pub resources: BTreeMap<String, Resource>,
    pub constraints: BTreeMap<String, Vec<u32>>,
    pub fitness: BTreeMap<String, u32>,
    mutation_ratio: u32,
}

impl KnapSack {
    pub fn load_custom_kp(config: &Config) -> Self {
        let mut knapsack: KnapSack = KnapSack::default();
        let data: String = fs::read_to_string(&config.path).expect("Unable to read the file");
        let lines = data.split('\n').collect::<Vec<&str>>();

        for v in lines {
            let arr = v.trim().split(':').collect::<Vec<&str>>();

            match arr[0].trim().to_lowercase().as_str() {
                "resource" => {
                    let r = Resource::new(&arr[1..]);
                    knapsack.resources.insert(arr[1].trim().to_string(), r);
                }
                "product" => {
                    let p = Product::new(&arr[1..]);
                    knapsack.products.insert(arr[1].trim().to_string(), p);
                }
                _ => (),
            }
        }

        knapsack.mutation_ratio = config.mutations_per_1k;
        knapsack.compute_constraints();
        knapsack
    }

    pub fn compute_constraints(&mut self) {
        // let mut constraints: Vec<Vec<u32>> = Vec::new();

        for p in self.products.values_mut() {
            let possible_max = p
                .requirements
                .iter()
                .filter(|q| q.amount > 0)
                .map(|q| match self.resources.get(&q.id.to_string()) {
                    None => 0,
                    Some(r) => r.amount / i64::from(q.amount),
                })
                .min()
                .expect("Something went wrong with product max");

            p.max = possible_max as u32;
            // println!("Found max to be {possible_max}");

            let iter_max = (possible_max as f64).log2().ceil() as u32;
            let offset = if 2_i64.pow(iter_max) == possible_max {
                1
            } else {
                0
            };

            // println!("Adding constraint from 2^0 to 2^{}", iter_max + offset - 1);
            for c in 0..(iter_max + offset) {
                let mut cx = Vec::new();
                for kr in self.resources.keys() {
                    let i = p.requirements.iter().find(|q| &q.id == kr);
                    match i {
                        Some(r) => cx.push(r.amount * 2_u32.pow(c)),
                        None => cx.push(0),
                    }
                }

                let key = format!("{}_{}", p.id, c);
                self.constraints.insert(key.clone(), cx);
                self.fitness.insert(key, p.value * 2_u32.pow(c));

                // let key = format!("{}_{}", p.id, c);
            }
        }
    }

    pub fn get_fitness(&self, indiv: &Individual) -> u32 {
        let mut total = 0_u32;

        for g in indiv.active_genes() {
            total += self.fitness[g];
        }

        total
    }

    fn validate(&self, indiv: &Individual) -> bool {
        let req = self.remains(indiv);

        req.iter().all(|(_k, v)| v.amount >= 0)
    }

    pub fn remains(&self, indiv: &Individual) -> BTreeMap<String, Resource> {
        let mut req = self.resources.clone();

        for active_gene in indiv.active_genes() {
            for (index, (_, resource)) in req.iter_mut().enumerate() {
                resource.amount -= i64::from(self.constraints[active_gene][index]);
            }
        }

        req
    }

    pub fn requires(&self, indiv: &Individual) -> BTreeMap<String, Resource> {
        let mut req = self.resources.clone();

        for active_gene in indiv.active_genes() {
            for (index, item) in self.resources.iter().enumerate() {
                let local_requirement = i64::from(self.constraints[active_gene][index]);

                req.entry(item.0.to_string())
                    .and_modify(|r| r.amount += local_requirement)
                    .or_insert(Resource {
                        id: item.1.id.to_string(),
                        title: item.1.title.to_string(),
                        amount: local_requirement,
                    });
            }
        }

        req
    }

    pub fn make_valid(&self, mut indiv: Individual) -> Individual {
        while !self.validate(&indiv) {
            indiv.mutate_down(self);
        }
        indiv.fitness = self.get_fitness(&indiv);
        indiv
    }

    pub fn cross_genes(&self, first: &Individual, other: &Individual) -> Individual {
        let keys = self.fitness.keys().collect::<Vec<_>>();

        let mut output = Individual::default();

        match rand::thread_rng().gen_range(1..=3) {
            1 => self.cross_1(keys, &mut output, first, other),
            2 => self.cross_2(keys, &mut output, first, other),
            3 => self.cross_rand(&mut output, first, other),
            _ => (),
        }

        if rand::thread_rng().gen_ratio(self.mutation_ratio, 1000) {
            for _ in 0..rand::thread_rng().gen_range(1..=self.fitness.len() / 2) {
                    output.mutate_up(self);                
            }
        }

        output = self.make_valid(output);

        output
    }

    fn cross_1(
        &self,
        keys: Vec<&String>,
        output: &mut Individual,
        first: &Individual,
        other: &Individual,
    ) {
        let k1 = rand::thread_rng().gen_range(0..(self.fitness.len()));

        for g in self.fitness.keys() {
            if g < keys[k1] {
                output
                    .genotype
                    .entry(g.to_string())
                    .or_insert(first.genotype[g]);
            } else {
                output
                    .genotype
                    .entry(g.to_string())
                    .or_insert(other.genotype[g]);
            }
        }
    }

    fn cross_2(
        &self,
        keys: Vec<&String>,
        output: &mut Individual,
        first: &Individual,
        other: &Individual,
    ) {
        let k1 = rand::thread_rng().gen_range(0..(self.fitness.len()));
        let k2 = rand::thread_rng().gen_range(0..(self.fitness.len()));
        for g in self.fitness.keys() {
            // if random() {
            if ((g < keys[k1]) & (g < keys[k2])) | ((g > keys[k1]) & (g > keys[k2])) {
                output
                    .genotype
                    .entry(g.to_string())
                    .or_insert(first.genotype[g]);
            } else {
                output
                    .genotype
                    .entry(g.to_string())
                    .or_insert(other.genotype[g]);
            }
        }
    }

    fn cross_rand(&self, output: &mut Individual, first: &Individual, other: &Individual) {
        for g in self.fitness.keys() {
            if rand::random() {
                output
                    .genotype
                    .entry(g.to_string())
                    .or_insert(first.genotype[g]);
            } else {
                output
                    .genotype
                    .entry(g.to_string())
                    .or_insert(other.genotype[g]);
            }
        }
    }

    pub fn explain_solution(
        &self,
        champion: &Individual,
        champion_generation: u32,
        config: &Config,
    ) {
        println!(
            "\n-------\nFound {}$ worth solution at gen {}, performing {:.2}%",
            champion.fitness,
            champion_generation,
            100_f64 * champion.fitness as f64 / config.known_best as f64
        );
        // println!("Best solution is {:?}", champion);
        let mut solution = self.products.clone();

        for c in champion.active_genes() {
            let p = parse_constraint(c);
            match p {
                Ok((key, pow)) => {
                    solution
                        .entry(key)
                        .and_modify(|p| p.solution += 2_u32.pow(pow));
                }
                Err(msg) => println!("{}", msg),
            }
        }

        println!("\nSolution\n-------");
        for (_, product) in solution {
            println!(
                "{} : {} ({}$) on {} ({}$)",
                product,
                product.solution,
                product.solution * product.value,
                product.max,
                product.max * product.value
            );
        }

        println!("\nRemains\n-------");
        let req = self.remains(champion);
        req.iter().for_each(|(k, v)| println!("{}: {:?}", k, v));
    }
}

impl fmt::Display for KnapSack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::from("Products\n");
        for p in &self.products {
            output = format!("{}- {}\n", output, p.1);
        }

        output += "Resources\n";
        for r in &self.resources {
            output = format!("{}- {}\n", output, r.1);
        }

        output += "Output matrix\n";
        for c in self.fitness.keys() {
            output = format!(
                "{}{}: {:?} => {}\n",
                output,
                c,
                self.constraints[c.as_str()],
                self.fitness[c]
            );
        }

        write!(f, "{}", output)
    }
}

fn parse_constraint(constraint: &str) -> Result<(String, u32), String> {
    let parts = constraint.split('_').collect::<Vec<_>>();
    match parts[..] {
        [prod, pow] => match pow.parse::<u32>() {
            Ok(p) => Ok((prod.to_string(), p)),
            Err(msg) => Err(msg.to_string()),
        },
        _ => Err(String::from("invalid constraint key")),
    }
}
