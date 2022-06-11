use std::marker::PhantomData;
use supper::*;
#[derive(Debug, Default)]
pub struct Equation<IterMethod> {
    current: u32,
    _methods: PhantomData<IterMethod>,
}
#[derive(Debug, Default)]
pub struct Linear;

#[derive(Debug, Default)]
pub struct Quadratic;

impl Iterator for Equation<Linear> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u32::MAX {
            return None;
        }
        Some(self.current)
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u16::MAX as  u32 {
            return None;
        }
        Some((self.current * self.current))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_linear(){
        let mut equation = Equation::<Linear>::Default();
        assert_eq!(Some(1),equation.next());
        assert_eq!(Some(2),equation.next());
        assert_eq!(Some(3),equation.next());
    }

    #[test]
    fn test_quadratic(){
        let mut equation = Equation::<Quadratic>::Default();
        assert_eq!(Some(1),equation.next());
        assert_eq!(Some(4),equation.next());
        assert_eq!(Some(9),equation.next());
    }
}
fn main() {
    
    
}


