use rand::Rng;

mod config;
use config::*;
mod solver;
use solver::*;

fn main() {
    let mut rng = rand::thread_rng();
    let config = Config::load_params().unwrap();
    let knapsack = KnapSack::load_custom_kp(&config);
    // println!("{}", knapsack);

    let mut pop = new_population(&config, &knapsack);

    let mut champion = pop[0].clone();
    let mut champion_generation = 0;
    let mut latest_generation = 0;

    for gen in 1..=config.generations_count {
        let mut new_pop = Vec::new();

        let total_fitness = pop
            .iter()
            .map(|p| p.fitness)
            .reduce(|total, fitness| total + fitness)
            .expect("Invalid total_fitness");

        if gen % config.frequency == 0 {
            println!(
                "Gen #{}, fitness:{} (ranging {}..{}) - current champion: {}",
                gen,
                total_fitness,
                pop[0].fitness,
                pop[pop.len() - 1].fitness,
                champion.fitness
            );
        }

        pop = distribute(&pop);
        // pop.iter().for_each(|i| print!("<{}>", i.fitness));
        // println!("--");

        while new_pop.len() < config.population_size {
            let pick1 = pick_individual(&mut rng, total_fitness, &pop);
            let first = &pop[pick1];
            let pick2 = pick_individual(&mut rng, total_fitness, &pop);
            let other = &pop[pick2];

            new_pop.push(knapsack.cross_genes(first, other));
        }

        new_pop.sort_by(|a, b| b.fitness.cmp(&a.fitness));

        if new_pop[0].fitness > champion.fitness {
            champion = new_pop[0].clone();
            champion_generation = gen;
            // println!("> new champion @gen #{}: {}$.", gen, champion.fitness);
        } else {
            if (gen - champion_generation > config.stability_threshold)
                & (gen - latest_generation > config.stability_threshold)
            {
                // println!("> too stable, new population @gen #{}.", gen);
                new_pop = new_population(&config, &knapsack);
                latest_generation = gen;
            }
        }
        new_pop.pop();
        new_pop.insert(0, champion.clone());

        pop = new_pop;
    }

    knapsack.explain_solution(&champion, champion_generation, &config);
}

fn new_population(config: &Config, knapsack: &KnapSack) -> Vec<Individual> {
    let mut pop = (0..config.population_size)
        .map(|_| Individual::new(knapsack))
        .collect::<Vec<_>>();
    pop.sort_by(|a, b| b.fitness.cmp(&a.fitness));
    pop
}

fn distribute(pop: &[Individual]) -> Vec<Individual> {
    let mut output = Vec::new();

    for (i, item) in pop.iter().enumerate() {
        match i % 2 {
            0 => output.push(item.clone()),
            1 => output.insert(0, item.clone()),
            _ => {
                println!("weird usize {}", i);
            }
        }
    }

    output
}

fn pick_individual(
    rng: &mut rand::rngs::ThreadRng,
    total_fitness: u32,
    pop: &[Individual],
) -> usize {
    if total_fitness == 0 {
        return 0_usize;
    }
    let elite_ratio = rng.gen_range(1..4);
    let pick_rng = rng.gen_range(0..total_fitness / elite_ratio);

    let mut cur_fitness = pop[0].fitness;
    let mut cur_pick = 0;

    while cur_fitness < pick_rng {
        cur_pick += 1;
        cur_fitness += pop[cur_pick].fitness;
    }

    cur_pick
}
