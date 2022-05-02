use rand::Rng;

type Weight = i32;
type Value = i32;
type Genome = Vec<i32>;
type Population = Vec<Genome>;
type Thing = (i32, i32);
type Things = Vec<Thing>;
//impl  Things {
//Thing: "Laptop", 500, 2200,
//Thing: "Laptop", 150, 160,
//Thing: "Laptop", 60, 350,
//Thing: "Laptop", 40, 333,
//Thing: "Water Bottle", 30, 192,
//}
fn generate_genome(length: usize) -> Genome {
    let mut choices = Vec::with_capacity(length);
    for _i in 0..length {
        let rng = rand::thread_rng().gen_range(0..2);
        choices.push(rng);
    }
    choices
}

fn generate_population(size: i32, genome_length: i32) -> Population {
    let mut population_pool: Population = Vec::new();

    for i in 0..size {
        population_pool.push(generate_genome(genome_length));
    }
    population_pool
}

fn fitness(genome: Genome, things: Things, weigth_limit: Weight) -> Value {
    if genome.capacity() != things.capacity() {
        panic!("Genome and things must be of the same length");
    }
    let mut weight = 0;
    let mut value = 0;
    for (_i, thing) in things.iter().enumerate() {
        if genome[1] == 1 {
            weight += thing.0;
            value += thing.1;
            if weight > weigth_limit {
                return 0;
            }
        }
    }
    value
}
fn selection_pair(population: Population, fitness_func: &dyn FnMut(Genome, i32)) -> Population {
    return;
}
fn main() {
    for i in 0..5 {
        println!("{:?}", generate_genome(i as i32));
    }
    println!("Hello, world!");
}
