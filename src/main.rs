mod cmd_input;
mod data_input;
mod alz;
extern crate rand;

use std::fs::File;
use alz::Population;
use alz::Probabilities;
use alz::Stats;

struct YearStats (u8, Stats);

fn main() {
    const SIM_DURATION_YEARS :  u8 = 70;

    let mut stats_history : Vec<YearStats> = Vec::with_capacity(SIM_DURATION_YEARS as usize);
    let mut population: Population = Population::from_cohorts(&data_input::numbers_from_file::<i32>(&mut File::open("population.txt").unwrap()));
    let probabilities: Probabilities = create_probabilities();
    let mut first_year_stats : Stats = Stats::of(&population);
    
    population.initialize(&probabilities);
    first_year_stats = Stats::of(&population);
    stats_history.push(YearStats(0, first_year_stats.clone()));

    println!("Year 0 preparations finished. Total population: {}. Healthy: {}. Ill: {}.", first_year_stats.total_population(), first_year_stats.total_healthy(), first_year_stats.total_ill());

    for year in 0..SIM_DURATION_YEARS{
        println!("Year {} started.", year + 1);
        iterate(&mut population, &probabilities, year);
        let stats = Stats::of(&population);
        stats_history.push(YearStats(year, stats.clone()));
        println!("Total population: {}. Healthy:{}. Ill {}. ",stats.total_population(), stats.total_healthy(), stats.total_ill());
        println!("--Year {} finished.", year + 1);
    }

    write_stats(&stats_history);
}

#[inline]
fn iterate(population: &mut Population, probabilities: &Probabilities, year: u8){
    let mut current_year_stats = Stats::of(&population);
    unsafe{
        population.death(probabilities, year, &current_year_stats);
    }
    population.birth(probabilities, year);
    current_year_stats = Stats::of(&population);
    unsafe{
        population.alzheimer(&current_year_stats, &probabilities);
    }



}

#[inline]
fn create_probabilities() -> Probabilities{
    let a: Vec<f64> = data_input::numbers_from_file::<f64>(&mut File::open("death_factor_a.txt").unwrap());
    let b: Vec<f64> = data_input::numbers_from_file::<f64>(&mut File::open("death_factor_b.txt").unwrap());
    let birth: Vec<f64> = data_input::numbers_from_file::<f64>(&mut File::open("birth_factor.txt").unwrap());
    let high_age : Vec<f64> = data_input::numbers_from_file::<f64>(&mut File::open("high_age_death_factor.txt").unwrap());
    let alz: Vec<f64> = data_input::numbers_from_file::<f64>(&mut File::open("alz_death_factor.txt").unwrap());
    let q99: Vec<f64> = data_input::numbers_from_file::<f64>(&mut File::open("q99.txt").unwrap());
    let init_population: Vec<u32> = data_input::numbers_from_file::<u32>(&mut File::open("population.txt").unwrap());
    let r0: Vec<f64> = data_input::numbers_from_file::<f64>(&mut File::open("r0.txt").unwrap());
    let r: Vec<f64> = data_input::numbers_from_file::<f64>(&mut File::open("r.txt").unwrap());

    let mut high_age_death_factor : Vec<f64> = Vec::with_capacity(100);
    for _ in 0..84{
        high_age_death_factor.push(0f64);
    }
    high_age_death_factor.extend(high_age);

    let mut i : u8 = 0;
    for number in high_age_death_factor.iter(){
        println!("Age {}: {}",i, number);
        i += 1;
    }
    Probabilities::new(a, b, birth, high_age_death_factor, alz, q99, init_population, r0, r)
}

fn write_stats(stats: &Vec<YearStats>){

    let mut csv_lines : Vec<String> = Vec::with_capacity(stats.len() + 1);
    let mut initial_line : String = String::new();
    initial_line.push_str("Year, 0H, 1H,  2H, 3H, 4H, 5H, 6H, 7H, 8H, 9H, 10H,    11H,    12H,    13H,    14H,    15H,    16H,    17H,    18H,    19H,    20H,    21H,    22H,    23H,    24H,    25H,    26H,    27H,    28H,    29H,    30H,    31H,    32H,    33H,    34H,    35H,    36H,    37H,    38H,    39H,    40H,    41H,    42H,    43H,    44H,    45H,    46H,    47H,    48H,    49H,    50H,    51H,    52H,    53H,    54H,    55H,    56H,    57H,    58H,    59H,    60H,    61H,    62H,    63H,    64H,    65H,    66H,    67H,    68H,    69H,    70H,    71H,    72H,    73H,    74H,    75H,    76H,    77H,    78H,    79H,    80H,    81H,    82H,    83H,    84H,    85H,    86H,    87H,    88H,    89H,    90H,    91H,    92H,    93H,    94H,    95H,    96H,    97H,    98H,    99H,    100H, 0A, 1A, 2A, 3A, 4A, 5A, 6A, 7A, 8A, 9A, 10A,    11A,    12A,    13A,    14A,    15A,    16A,    17A,    18A,    19A,    20A,    21A,    22A,    23A,    24A,    25A,    26A,    27A,    28A,    29A,    30A,    31A,    32A,    33A,    34A,    35A,    36A,    37A,    38A,    39A,    40A,    41A,    42A,    43A,    44A,    45A,    46A,    47A,    48A,    49A,    50A,    51A,    52A,    53A,    54A,    55A,    56A,    57A,    58A,    59A,    60A,    61A,    62A,    63A,    64A,    65A,    66A,    67A,    68A,    69A,    70A,    71A,    72A,    73A,    74A,    75A,    76A,    77A,    78A,    79A,    80A,    81A,    82A,    83A,    84A,    85A,    86A,    87A,    88A,    89A,    90A,    91A,    92A,    93A,    94A,    95A,    96A,    97A,    98A,    99A,    100A, TOTAL HEALTHY, TOTAL ILL, TOTAL POPULATION");
    csv_lines.push(initial_line);

    for year_stat in stats {
        let mut line : String = String::new();
        line.push_str(&year_stat.0.to_string());
        line.push_str(",");
        line.push_str(&year_stat.1.csv());
        csv_lines.push(line);
    }

    data_input::write_vec_to_file(&mut File::create("alz_results.txt").unwrap(), &csv_lines);
}