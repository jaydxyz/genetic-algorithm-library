use std::fmt::Debug;
use std::marker::PhantomData;
use rayon::prelude::*;
use rand::prelude::*;

pub trait Individual: Clone + Debug + Send + Sync {
    fn fitness(&self) -> f64;
    fn crossover(&self, other: &Self) -> (Self, Self);
    fn mutate(&mut self);
}

pub struct GeneticAlgorithm<I, S, C, M>
where
    I: Individual,
    S: SelectionStrategy<I>,
    C: CrossoverOperator<I>,
    M: MutationOperator<I>,
{
    population_size: usize,
    selection_strategy: S,
    crossover_operator: C,
    mutation_operator: M,
    _marker: PhantomData<I>,
}

impl<I, S, C, M> GeneticAlgorithm<I, S, C, M>
where
    I: Individual,
    S: SelectionStrategy<I>,
    C: CrossoverOperator<I>,
    M: MutationOperator<I>,
{
    pub fn new(
        population_size: usize,
        selection_strategy: S,
        crossover_operator: C,
        mutation_operator: M,
    ) -> Self {
        GeneticAlgorithm {
            population_size,
            selection_strategy,
            crossover_operator,
            mutation_operator,
            _marker: PhantomData,
        }
    }

    pub fn evolve<R: Rng + Send + Sync>(
        &self,
        generations: usize,
        rng: &mut R,
        initial_population: Option<Vec<I>>,
    ) -> Vec<I> {
        let mut population = match initial_population {
            Some(pop) => pop,
            None => (0..self.population_size)
                .into_par_iter()
                .map(|_| self.generate_individual(rng))
                .collect(),
        };

        for _ in 0..generations {
            let fitness_values: Vec<f64> = population
                .par_iter()
                .map(|individual| individual.fitness())
                .collect();

            let parents = self
                .selection_strategy
                .select(&population, &fitness_values, rng);

            let offspring: Vec<I> = parents
                .par_chunks(2)
                .flat_map(|pair| {
                    if pair.len() == 2 {
                        self.crossover_operator.crossover(&pair[0], &pair[1])
                    } else {
                        vec![pair[0].clone()]
                    }
                })
                .collect();

            population = offspring
                .into_par_iter()
                .map(|mut individual| {
                    self.mutation_operator.mutate(&mut individual, rng);
                    individual
                })
                .collect();
        }

        population
    }

    fn generate_individual<R: Rng>(&self, _rng: &mut R) -> I {
        // This method should be implemented by the user when creating a specific GeneticAlgorithm instance
        unimplemented!("generate_individual must be implemented for the specific Individual type")
    }
}

pub trait SelectionStrategy<I: Individual>: Send + Sync {
    fn select<R: Rng>(&self, population: &[I], fitness_values: &[f64], rng: &mut R) -> Vec<I>;
}

pub trait CrossoverOperator<I: Individual>: Send + Sync {
    fn crossover(&self, parent1: &I, parent2: &I) -> Vec<I>;
}

pub trait MutationOperator<I: Individual>: Send + Sync {
    fn mutate<R: Rng>(&self, individual: &mut I, rng: &mut R);
}

// Example implementation of selection strategy: Tournament Selection
pub struct TournamentSelection {
    tournament_size: usize,
}

impl<I: Individual> SelectionStrategy<I> for TournamentSelection {
    fn select<R: Rng>(&self, population: &[I], fitness_values: &[f64], rng: &mut R) -> Vec<I> {
        (0..population.len())
            .map(|_| {
                let tournament = (0..self.tournament_size)
                    .map(|_| rng.gen_range(0..population.len()))
                    .collect::<Vec<_>>();
                tournament
                    .into_iter()
                    .max_by(|&a, &b| fitness_values[a].partial_cmp(&fitness_values[b]).unwrap())
                    .map(|index| population[index].clone())
                    .unwrap()
            })
            .collect()
    }
}

// Example usage
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
            genes: self.genes[..mid]
                .iter()
                .chain(&other.genes[mid..])
                .cloned()
                .collect(),
        };
        let child2 = MyIndividual {
            genes: other.genes[..mid]
                .iter()
                .chain(&self.genes[mid..])
                .cloned()
                .collect(),
        };
        (child1, child2)
    }

    fn mutate(&mut self) {
        if let Some(gene) = self.genes.iter_mut().choose(&mut thread_rng()) {
            *gene += thread_rng().gen_range(-0.1..0.1);
        }
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
