use rand::distributions::*;
use rand::prelude::*;
use rand::seq;
use rand::Rng;

type Weight = i32;
type Value = i32;
type Chromosome = Vec<i32>;
type Population = Vec<Chromosome>;
type Thing = (Value, Weight);
type Things = Vec<Thing>;
type FitnessFunc = fn(Chromosome) -> Value;

fn generate_genome(length: usize) -> Chromosome {
    let mut genome_seq = Vec::with_capacity(length);
    for _i in 0..length {
        let gen = rand::thread_rng().gen_range(0..2);
        genome_seq.push(gen);
    }
    genome_seq
}

fn generate_population(size: i32, length: usize) -> Population {
    let mut population_pool: Population = Vec::new();

    for _ in 0..size {
        population_pool.push(generate_genome(length));
    }
    population_pool
}

fn fitness(individual: Chromosome, things: Things, weigth_limit: Weight) -> Value {
    if individual.capacity() != things.capacity() {
        panic!("Genome and things must be of the same length");
    }
    let mut weight = 0;
    let mut value = 0;
    for (i, thing) in things.iter().enumerate() {
        if individual[i] == 1 {
            weight += thing.0;
            value += thing.1;
            if weight > weigth_limit {
                return 0;
            }
        }
    }
    value
}
// need to select two random individuals from my population while considering the weight of each
// person
fn selection_pair(population: Population, fitness_func: FitnessFunc) -> Population {
    let mut rng = rand::thread_rng();
    let mut weights = Vec::new();

    for individual in population {
        let weight = fitness_func(individual);
        weights.push(weight);
    }
    population
}
fn single_point_crossover(individual: Chromosome, individual: Chromosome) -> (Chromosome, Chromosome) {
    if individual.capacity() != individual.capacity() {
        panic!("Genomes a and b must be of the same length");
    }
    let length = individual.capacity();
    if length < 2 {
        return (individual, individual);
    }
    let rng = SeedableRng::from_rng(thread_rng()).unwrap();
    let p = rng.gen_range(0..length - 1) as u8;
    return (individual[0:p] + individual[p:end], individual[0:p] + individual[p:end]) 
}

fn  mutation(individual: Chromosome, )
fn main() {
    for i in 0..5 {
        println!("{:?}", generate_genome(i));
    }
    println!("Hello, world!");
}
