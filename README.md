# Rust Genetic Algorithm Library

A highly optimized and generic genetic algorithm library implemented in Rust. This library is designed to solve various optimization problems using genetic algorithms, showcasing advanced Rust concepts such as generics, trait bounds, and iterators for a flexible and efficient implementation.

## Features

- Generic representation of individuals, allowing users to define custom data structures for their specific problem.
- Flexible fitness function definition using traits, enabling customization based on the problem requirements.
- Various selection strategies, including tournament selection, roulette wheel selection, and rank-based selection.
- Customizable crossover and mutation operators that can be easily extended to suit specific problem needs.
- Parallel processing using Rust's concurrency features (`rayon` or `std::thread`) for improved performance.
- Termination criteria specification, such as maximum number of generations or desired fitness threshold.
- Methods to collect and report statistics, such as best fitness, average fitness, and population diversity.
- Well-documented examples demonstrating how to use the library for solving different optimization problems.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
rust-genetic-algorithm = "0.1.0"
```

Then, run `cargo build` to build the library.

## Usage

Here's a basic example of how to use the Rust Genetic Algorithm Library:

```rust
use rust_genetic_algorithm::{GeneticAlgorithm, Individual};

struct MyIndividual {
    // Define the structure of an individual
}

impl Individual for MyIndividual {
    fn fitness(&self) -> f64 {
        // Implement the fitness function
        unimplemented!()
    }

    fn crossover(&self, other: &Self) -> (Self, Self) {
        // Implement the crossover operation
        unimplemented!()
    }

    fn mutate(&mut self) {
        // Implement the mutation operation
        unimplemented!()
    }
}

fn main() {
    let population_size = 100;
    let generations = 50;

    let selection_strategy = /* Implement a specific selection strategy */;
    let crossover_operator = /* Implement a specific crossover operator */;
    let mutation_operator = /* Implement a specific mutation operator */;

    let ga = GeneticAlgorithm::new(
        population_size,
        selection_strategy,
        crossover_operator,
        mutation_operator,
    );

    let mut rng = thread_rng();
    let best_individual = ga.evolve(generations, &mut rng)
        .into_iter()
        .max_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap())
        .unwrap();

    println!("Best individual: {:?}", best_individual);
}
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

## License

This library is licensed under the [MIT License](LICENSE).
