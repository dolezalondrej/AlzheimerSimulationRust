extern crate rand;
use std::vec::Vec;
use rand::Rng;
use std::thread;
use std::f64;
use std::clone::Clone;
use std::sync::Arc;


pub struct Stats{
	ill_cohorts: Vec<i32>,
	healthy_cohorts: Vec<i32>
}

impl Stats{
	pub fn new(population: &Vec<Man>) -> Stats {
		let mut statistics : Stats = Stats{ill_cohorts: Vec::with_capacity(101), healthy_cohorts: Vec::with_capacity(101)};
		for _ in 0..statistics.healthy_cohorts.capacity(){
			statistics.healthy_cohorts.push(0);
			statistics.ill_cohorts.push(0);
		}

		for man in population.iter(){
			if man.alzheimer {
				statistics.ill_cohorts[man.age as usize] += 1;
			} else {
				statistics.healthy_cohorts[man.age as usize] += 1;

			}
		}

		statistics
	}
	pub fn of(population: &Population) -> Stats{
		Stats::new(&population.people)

	}

	pub fn total_population(&self) -> i32 {
		self.total_healthy() + self.total_ill()
	}

	pub fn total_cohort(&self, cohort: usize) -> i32{
		self.ill_cohorts[cohort] + self.healthy_cohorts[cohort]
	}

	pub fn total_healthy(&self) -> i32{
		self.healthy_cohorts.iter().fold(0, |x,y| x + y)
	}

	pub fn total_ill(&self) -> i32{
		self.ill_cohorts.iter().fold(0, |x, y| x + y)
	}

	pub fn csv(&self) -> String {
		let mut csv : String = String::new();

		for cohort in 0..self.healthy_cohorts.len(){
			csv.push_str(&self.healthy_cohorts[cohort].to_string());
			csv.push_str(",");
		}
		for cohort in 0..self.ill_cohorts.len(){
			csv.push_str(&self.ill_cohorts[cohort].to_string());
			csv.push_str(",");
		}

		csv.push_str(&self.total_healthy().to_string());
		csv.push_str(",");
		csv.push_str(&self.total_ill().to_string());
		csv.push_str(",");
		csv.push_str(&self.total_population().to_string());

		return csv;
	}
}

impl Clone for Stats{

	fn clone(&self) -> Stats {
		Stats{ill_cohorts: self.ill_cohorts.clone(), healthy_cohorts: self.healthy_cohorts.clone()}

	}

	fn clone_from(&mut self, source: &Self) { 
		self.healthy_cohorts = source.healthy_cohorts.clone();
		self.ill_cohorts = source.ill_cohorts.clone();
	}
}

pub struct Population{
	people: Vec<Man>
}

impl Population{

	fn with_capacity(capacity: usize) -> Population{
		Population{people: Vec::with_capacity(capacity)}
	}

	pub fn from_cohorts(cohorts: &Vec<i32>) -> Population{
		let num_people: i32 = cohorts.iter().fold(0,|x,y| {x + y});
		let mut population: Population = Population::with_capacity(527000000);

		for age in 0..cohorts.len(){
			for _ in 0..cohorts[age]{
				population.people.push(Man::new(age as u8));
			}
		}
		population

	}

	pub fn birth(&mut self,probabilities: &Probabilities, year: u8){
		
		let mut birth : f64 =  ((self.people.len() as f64) * probabilities.birth[year as usize]) ;

		for _ in 0..birth as u64{
			self.people.push(Man::newborn());
		}

		println!("Born {}.", birth);
	}


	pub unsafe fn death(&mut self, probabilities: &Probabilities, year : u8, stats: &Stats){

		let chunk_size : usize = (self.people.len() as f64 / (12 as f64)).ceil() as usize;
		//A little bit of a compiler hack. In Rust version 1.0.0 stable, there is no JoinGuard.
		let mut _people: *mut Vec<Man> = &mut self.people as *mut Vec<Man>;
		let dereferenced_people : &mut Vec<Man> = &mut *_people;
		let ref_stats : *const Stats = stats as *const Stats;
		let _stats : &Stats = & * ref_stats;
		let ref_probabilities : *const Probabilities = probabilities as *const Probabilities;
		let _probabilities : &Probabilities = & * ref_probabilities;

		let mut thread_handles = Vec::new();

		for chunk in dereferenced_people.chunks_mut(chunk_size){
			let handle = thread::spawn(move || {
				let mut random = rand::thread_rng();

				for man in chunk.iter_mut(){

					let mut death_prob : f64;
					if man.alzheimer { 
						death_prob = _probabilities.alz_death[(man.age - 65) as usize];
					} else{


						if man.age < 85 { 
							death_prob = _probabilities.a[man.age as usize] / (year as f64 + _probabilities.b[man.age as usize])
						} 
						else {
							if man.age == 99 {
								death_prob = _probabilities.q99[year as usize];
							} else{
								death_prob = _probabilities.high_age_death[man.age as usize];
							}
						};

						if  _stats.ill_cohorts[man.age as usize] > 0 {
							let num_people_age : f64 = _stats.total_cohort(man.age as usize) as f64;
							death_prob =  (num_people_age * death_prob - (_stats.ill_cohorts[man.age as usize] as f64 * _probabilities.alz_death[(man.age - 65) as usize])) / (_stats.healthy_cohorts[man.age as usize] as f64);
						}
					}

					if random.next_f64() <= death_prob || man.age >= 100{
						man.alive = false;
					}
					man.age += 1;

				}

			});
thread_handles.push(handle);

}


for thread in thread_handles.into_iter(){
	thread.join().unwrap();
}
let old = self.people.len();
self.people.retain(|m| m.alive);
println!("Died {}.", old - self.people.len());
}

pub fn initialize(&mut self, probabilities: &Probabilities){
	for cohort in 65u8..101u8{
		let rx: u32 = (probabilities.r0[cohort as usize] * (probabilities.init_population[cohort as usize] as f64)) as u32;
		println!("Initial number of ALZ people aged {} is {}", cohort, rx);

		let mut converted : u32 = 0;
		for man in self.people.iter_mut(){
			if converted == rx {break;}

			if !man.alzheimer && man.age == cohort {
				man.alzheimer = true;
				converted += 1;
			}

		}

	}
}

pub unsafe fn alzheimer(&mut self, stats: &Stats, probabilities: &Probabilities){

	let chunk_size : usize = (self.people.len() as f64 / 12 as f64).ceil() as usize;
		//A little bit of a compiler hack. In Rust version 1.0.0 stable, there is no JoinGuard.
		let mut _people: *mut Vec<Man> = &mut self.people as *mut Vec<Man>;
		let dereferenced_people : &mut Vec<Man> = &mut *_people;
		let _stats : *const Stats = stats as *const Stats;
		let mut alz_prob_cohorts = Vec::<f64>::with_capacity(35);

		for cohort in 65..101{

			let alz_prob_age: f64 = ((probabilities.r[cohort - 65] * (stats.total_cohort(cohort) as f64)) - (stats.ill_cohorts[cohort] as f64)) / (stats.healthy_cohorts[cohort] as f64);
			alz_prob_cohorts.push(alz_prob_age);
		}

		let alz_prob = Arc::new(alz_prob_cohorts);

		let mut thread_handles = Vec::new();

		for chunk in dereferenced_people.chunks_mut(chunk_size){
			let local_alz_prob = alz_prob.clone();
			let handle = thread::spawn(move ||{
				let mut random = rand::thread_rng();

				for man in chunk.iter_mut(){
					if man.age > 100 {continue;}

					if man.age > 64 && !man.alzheimer{
						let alzheimer_probability : f64 = local_alz_prob[man.age as usize - 65];

						if random.next_f64() <= alzheimer_probability{
							man.alzheimer = true;
						}
					}

				}

			});
			thread_handles.push(handle);
		}


		for thread in thread_handles.into_iter(){
			thread.join().unwrap();
		}


	}

}

pub struct Man{
	age: u8,
	alzheimer: bool,
	alive: bool
}

impl Man{
	pub fn new(age: u8) -> Man{
		Man{age: age, alzheimer: false, alive: true}
	}

	pub fn newborn() -> Man{
		Man::new(0)
	}
}

impl Clone for Man{
	fn clone(&self) -> Man{
		Man{age: self.age, alzheimer: self.alzheimer, alive: self.alive}
	}
}

pub struct Probabilities{
	a: Vec<f64>,
	b: Vec<f64>,
	birth: Vec<f64>,
	high_age_death : Vec<f64>,
	alz_death : Vec<f64>,
	q99: Vec<f64>,
	init_population: Vec<u32>,
	r0: Vec<f64>,
	r: Vec<f64>

}

impl Probabilities{
	pub fn new(a: Vec<f64>,b: Vec<f64>,birth: Vec<f64>,high_age_death: Vec<f64>, alz_death : Vec<f64>, q99: Vec<f64>, init_population: Vec<u32>, r0: Vec<f64>, r: Vec<f64>) -> Probabilities{
		Probabilities{a: a, b :b, birth: birth, high_age_death: high_age_death, alz_death: alz_death, q99: q99, init_population: init_population, r0: r0, r: r}
	}

}