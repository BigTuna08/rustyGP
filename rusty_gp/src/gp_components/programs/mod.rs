use ::gp_components::traits::Program;

pub struct SimpleProg {
    pub genotype: Vec<u8>,
    pub fitness: f32
}

impl Program for SimpleProg {
    fn genotype(&self) -> &Vec<u8> { &self.genotype }
    fn mut_genotype(&mut self) -> &mut Vec<u8> { &mut self.genotype}
    fn get_fitness(&self) -> f32 { self.fitness }
    fn set_fitness(&mut self, value: f32) -> () {self.fitness = value;}
}

