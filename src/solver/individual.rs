use rand::prelude::*;
use std::collections::BTreeMap;

use super::KnapSack;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Individual {
    pub genotype: BTreeMap<String, bool>,
    pub fitness: u32,
    // pub valid: bool,
}

fn rand_mutation_key(keys: Vec<&str>) -> String {
    let pos = rand::thread_rng().gen_range(0..(keys.len()));
    keys[pos].to_string()
}

impl Individual {
    fn _new(problem: &KnapSack) -> Self {
        let genes: BTreeMap<String, bool> = problem
            .fitness
            .keys()
            .map(|k| (k.to_string(), random()))
            .collect();

        Self {
            genotype: genes,
            fitness: 0,
        }
    }

    pub fn new(problem: &KnapSack) -> Self {
        let n = Self::_new(problem);

        problem.make_valid(n)
    }

    fn mutate(&mut self, key: String, value: bool) {
        self.genotype.entry(key).and_modify(|v| *v = value);
    }

    pub fn mutate_up(&mut self, problem: &KnapSack) {
        if random::<bool>() & random::<bool>() {
            self.rand_mutate_up();
        } else {
            self.focused_mutate_up(problem);
        }
    }

    pub fn mutate_down(&mut self, problem: &KnapSack) {
        if random::<bool>() & random::<bool>() {
            self.rand_mutate_down();
        } else {
            self.focused_mutate_down(problem);
        }
    }

    fn rand_mutate_down(&mut self) {
        let keys = self.active_genes().collect::<Vec<_>>();
        if !keys.is_empty() {
            let key = rand_mutation_key(keys);
            self.mutate(key, false);
        }
    }

    fn rand_mutate_up(&mut self) {
        let keys = self.inactive_genes().collect::<Vec<_>>();
        if !keys.is_empty() {
            let key = rand_mutation_key(keys);
            self.mutate(key, false);
        }
    }

    fn focused_mutate_up(&mut self, problem: &KnapSack) {
        let keys = self.inactive_genes().collect::<Vec<_>>();
        let req = problem.remains(self);

        let mut possible_opts = Vec::new();
        for k in keys {
            let constraint = &problem.constraints[k];
            if req
                .iter()
                .enumerate()
                .all(|(index, (_, r))| r.amount > constraint[index] as i64)
            {
                possible_opts.push(k);
            }
        }

        if !possible_opts.is_empty() {
            let pick = rand::thread_rng().gen_range(0..possible_opts.len());
            self.mutate(possible_opts[pick].to_string(), true);
        }
    }

    fn focused_mutate_down(&mut self, problem: &KnapSack) {
        let keys = self.active_genes().collect::<Vec<_>>();
        let req = problem.remains(self);

        let mut possible_opts = Vec::new();
        for k in keys {
            let constraint = &problem.constraints[k];
            if req
                .iter()
                .enumerate()
                .all(|(index, (_, r))| r.amount + constraint[index] as i64 >= 0)
            {
                possible_opts.push(k);
            }
        }

        if !possible_opts.is_empty() {
            let pick = rand::thread_rng().gen_range(0..possible_opts.len());
            self.mutate(possible_opts[pick].to_string(), true);
        }
    }

    pub fn active_genes(&self) -> impl Iterator<Item = &str> {
        self.genotype
            .iter()
            .filter(|(_k, g)| **g)
            .map(|(k, _)| k.as_str())
    }

    pub fn inactive_genes(&self) -> impl Iterator<Item = &str> {
        self.genotype
            .iter()
            .filter(|(_k, g)| !**g)
            .map(|(k, _)| k.as_str())
    }

    // fn enforce_valid_smart(&mut self) {
    //     while !self.is_valid() {
    //         let requires = self.requires();
    //         let mut worst_lack = requires.keys().collect::<Vec<_>>()[0];
    //         let pb = self.problem.unwrap();
    //         let mut worst_gap =
    //             i64::from(pb.resources[worst_lack].amount) - i64::from(requires[worst_lack]);
    //         for (k, v) in &requires {
    //             if (i64::from(pb.resources[k].amount) - i64::from(*v)) < worst_gap {
    //                 worst_gap = i64::from(pb.resources[k].amount) - i64::from(*v);
    //                 worst_lack = k;
    //             }
    //         }
    //         let idx = pb
    //             .resources
    //             .keys()
    //             .position(|r| r == worst_lack)
    //             .expect("The resource wasn't found");
    //         let mut all_prios = self
    //             .genotype
    //             .iter()
    //             .filter(|(_k, g)| **g)
    //             .map(|(k, _)| (k, pb.constraints[k][idx]))
    //             // .cmp(|c1, c2| c2.cmp(&c1))
    //             // .map(|(k, _)| k)
    //             .collect::<Vec<_>>();
    //         all_prios.sort_by(|(_, c1), (_, c2)| c1.cmp(&c2));
    //         let mut prio = all_prios
    //             .iter()
    //             // .filter(|(k, c)| *c >= worst_gap)
    //             .map(|(k, _)| *k)
    //             .collect::<Vec<_>>();
    //         prio.shuffle(&mut rand::thread_rng());
    //         self.genotype
    //             .entry(prio[0].to_string())
    //             .and_modify(|v| *v = false);
    //         self.valid = self.is_valid();
    //     }
    // }
}
