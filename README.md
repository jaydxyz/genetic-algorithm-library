# Rust Genetic Algorithm Library

A highly optimized and generic genetic algorithm library implemented in Rust. This library is designed to solve various optimization problems using genetic algorithms, showcasing advanced Rust concepts such as generics, trait bounds, and parallel iterators for a flexible and efficient implementation.

## Features

- Generic representation of individuals, allowing users to define custom data structures for their specific problem.
- Flexible fitness function definition using traits, enabling customization based on the problem requirements.
- Customizable selection strategies, crossover operators, and mutation operators that can be easily extended to suit specific problem needs.
- Parallel processing using Rayon for improved performance on multi-core systems.
- Option to provide an initial population or generate one randomly.
- Well-documented examples demonstrating how to use the library for solving different optimization problems.
- Thread-safe implementation, allowing for concurrent execution of genetic operations.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rust-genetic-algorithm = "0.2.0"
rayon = "1.5"
rand = "0.8"
```

Then, run `cargo build` to build the library.

## Usage

Here's a basic example of how to use the Rust Genetic Algorithm Library:

```rust
use rust_genetic_algorithm::{GeneticAlgorithm, Individual, SelectionStrategy, CrossoverOperator, MutationOperator};
use rand::prelude::*;

#[derive(Clone, Debug)]
struct MyIndividual {
    genes: Vec<f64>,
}

impl Individual for MyIndividual {
    fn fitness(&self) -> f64 {
        self.genes.iter().sum()
    }

    fn crossover(&self, other: &Self) -> (Self, Self) {
        let mid = self.genes.len() / 2;
        let child1 = MyIndividual {
            genes: self.genes[..mid].iter().chain(&other.genes[mid..]).cloned().collect(),
        };
        let child2 = MyIndividual {
            genes: other.genes[..mid].iter().chain(&self.genes[mid..]).cloned().collect(),
        };
        (child1, child2)
    }

    fn mutate(&mut self) {
        if let Some(gene) = self.genes.iter_mut().choose(&mut thread_rng()) {
            *gene += thread_rng().gen_range(-0.1..0.1);
        }
    }
}

struct TournamentSelection {
    tournament_size: usize,
}

impl<I: Individual> SelectionStrategy<I> for TournamentSelection {
    fn select<R: Rng>(&self, population: &[I], fitness_values: &[f64], rng: &mut R) -> Vec<I> {
        // Implementation details...
    }
}

struct SinglePointCrossover;

impl<I: Individual> CrossoverOperator<I> for SinglePointCrossover {
    fn crossover(&self, parent1: &I, parent2: &I) -> Vec<I> {
        vec![parent1.crossover(parent2).0, parent1.crossover(parent2).1]
    }
}

struct GaussianMutation {
    mutation_rate: f64,
    mutation_strength: f64,
}

impl<I: Individual> MutationOperator<I> for GaussianMutation {
    fn mutate<R: Rng>(&self, individual: &mut I, _rng: &mut R) {
        if thread_rng().gen_bool(self.mutation_rate) {
            individual.mutate();
        }
    }
}

fn main() {
    let population_size = 100;
    let generations = 50;
    let gene_length = 10;

    let selection_strategy = TournamentSelection { tournament_size: 3 };
    let crossover_operator = SinglePointCrossover;
    let mutation_operator = GaussianMutation {
        mutation_rate: 0.1,
        mutation_strength: 0.1,
    };

    let ga = GeneticAlgorithm::new(
        population_size,
        selection_strategy,
        crossover_operator,
        mutation_operator,
    );

    let initial_population: Vec<MyIndividual> = (0..population_size)
        .map(|_| MyIndividual {
            genes: (0..gene_length).map(|_| thread_rng().gen()).collect(),
        })
        .collect();

    let mut rng = thread_rng();
    let final_population = ga.evolve(generations, &mut rng, Some(initial_population));

    let best_individual = final_population
        .into_iter()
        .max_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap())
        .unwrap();

    println!("Best individual: {:?}", best_individual);
    println!("Fitness: {}", best_individual.fitness());
}
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This library is licensed under the [MIT License](LICENSE).
