use ::gp_components::traits::Population;
use ::gp_components::traits::Program;

pub struct SimplePop<T: Program>{
    pub members: Vec<T>
}


impl<T: Program> Population<T> for SimplePop<T> {
    fn members(&self) -> &Vec<T> {&self.members}
    fn mut_members(&mut self) -> &mut Vec<T> {&mut self.members}
}