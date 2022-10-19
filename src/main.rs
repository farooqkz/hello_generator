pub mod consts;
pub mod individual;
pub mod sinewave;
pub mod waveform;

use clap::{arg, command, Command};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use tinyrand::StdRand;
use vosk::{Model, Recognizer};
use std::fs::File;
use std::path::Path;
use wav::bit_depth::*;
use rayon::prelude::*;

fn cli() -> Command {
    command!().args([
        arg!(--model <PATH> "Path to model"),
        arg!(--pop <POPULATION_SIZE> "Population size(integer)"),
        arg!(--mutation <MUTATION_RATE> "Mutation rate(float)"),
        arg!(--maxgen <MAX_GEN> "Maximum generation to run(integer)"),
    ])
}

fn main() {
    let mut rng = StdRand::default();
    let matches = cli().get_matches();
    let model_path: String = (*matches
        .get_one::<String>("model")
        .expect("Please enter a path to the model"))
    .clone();
    let model: Model = Model::new(model_path).unwrap();
    let population_size: &str = (*matches
        .get_one::<String>("pop")
        .expect("Please enter population size"))
    .as_str();
    let population_size: u32 =
        u32::from_str_radix(population_size, 10).expect("Population size must be u32");
    let mutation_rate: &str = (*matches
        .get_one::<String>("mutation")
        .expect("Please enter the mutation rate"))
    .as_str();
    let mutation_rate: f32 = f32::from_str(mutation_rate).expect("Enter a float mutation rate");
    let mut population: Vec<(f32, crate::individual::Individual)> = vec![];
    let max_gen: &str = (*matches
        .get_one::<String>("maxgen")
        .expect("Please enter a maximum generation"))
    .as_str();
    let max_gen: u32 = u32::from_str_radix(max_gen, 10).expect("Please enter a u32 max gen");
    if population_size % 2 == 1 || population_size <= 0{
        panic!("population size MUST be even and positive");
    }
    let mut recognizer: Recognizer =
        Recognizer::new(&model, crate::consts::WAVE_FREQ as f32).expect("Cannot build recognizer");
    recognizer.set_max_alternatives(2);
    recognizer.set_partial_words(false);

    println!("Generating initial population");
    for _ in 0..population_size {
        let ind = crate::individual::Individual::new_rand(&mut rng);
        population.push((ind.fitness(&mut recognizer), ind));
    }
    println!("Generated initial population");
    for generation in 0..max_gen {
        population.par_sort_unstable_by(|ind0, ind1| ind0.0.total_cmp(&ind1.0));
        println!(
            "{} -- Gen {}. Best ft: {}. Worst ft: {}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            generation,
            population[0].0,
            population.last().unwrap().0
        );
        let mut offsprings: Vec<(f32, crate::individual::Individual)> = vec![];
        for pair in population.chunks_exact(2) {
            if let [(_ft0, ind0), (_ft1, ind1)] = pair {
                let children = ind0.combine(ind1, &mut rng);
                offsprings.push((children.0.fitness(&mut recognizer), children.0));
                offsprings.push((children.1.fitness(&mut recognizer), children.1));
            }
        }
        population.append(&mut offsprings);
        population.sort_unstable_by(|ind0, ind1| ind0.0.total_cmp(&ind1.0));
        population.truncate(population_size as usize);
        for (_ft, ind) in population.iter_mut() {
            ind.mutate(mutation_rate, &mut rng);
        }
    }
    println!("Writing best individual to ./result.wav");
    let mut outfile = File::create(Path::new("./result.wav")).expect("Cannot open file for writing");
    let header = wav::header::Header::new(
        wav::header::WAV_FORMAT_PCM,
        1,
        crate::consts::WAVE_FREQ as u32,
        16
    );
    wav::write(header, &wav::bit_depth::BitDepth::from(population[0].1.to_wave().samples.to_vec()), &mut outfile).expect("Cannot write wave");
}
