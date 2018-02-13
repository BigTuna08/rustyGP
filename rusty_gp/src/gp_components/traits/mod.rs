pub trait Program {
    fn genotype(&self) -> &Vec<u8>; //gives a reference to the programs genotype
    fn mut_genotype(&mut self) -> &mut Vec<u8>; //give mutable ref
    fn get_fitness(&self) -> f32;
    fn set_fitness(&mut self, f32) -> ();
}


//The type T of the population is the Program type
pub trait Population<T: Program> {
    fn members(&self) -> &Vec<T>;
    fn mut_members(&mut self) -> &mut Vec<T>;
}


//The type T of the initializer is the type of Population
pub trait PopulationInitializer<U: Program, T: Population<U>> {
    fn new_population(&self) -> T;
}


