use std::marker::PhantomData;

#[derive(Debug, Default)]
pub struct Equation<IterMethod> {
    current: u32,
    _method: PhantomData<IterMethod>, 
}

// 线性增长
#[derive(Debug, Default)]
pub struct Linear;

// 二次增长
#[derive(Debug, Default)]
pub struct Quadratic;

impl Iterator for Equation<Linear> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        println!("============{}================================", self.current);
        if self.current >= u32::MAX {
            return None;
        }
        Some(self.current)  
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;
    //&mut self,引用实此Iterator的类型实例
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        println!("-----------{}-----------", self.current);
        if self.current >= u32::MAX {
            return None;
        }

        Some(self.current * self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        // 可变量equation其类型为Equation，为Equation参数泛型为 Linear的实体结构的默认值，值为{current: 0, _method: PhantomData}
        let mut equation = Equation::<Linear>::default();
        println!("{:?}",equation);
        println!("{}",equation.current);
        assert_eq!(Some(1), equation.next());
        assert_eq!(Some(2), equation.next());
        assert_eq!(Some(3), equation.next());
    }

    #[test]
    fn test_quadratic() {
        let mut equation = Equation::<Quadratic>::default();
        assert_eq!(Some(1), equation.next());
        assert_eq!(Some(4), equation.next());
        assert_eq!(Some(9), equation.next());
    }
}
fn main() {

}

pub struct Kvpair {
    _key: String,
    _value: String,
}

pub struct KvError;
pub trait Storage {
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item =Kvpair>>,KvError>;
    fn get_anothoer_iter(&self, table: &str) -> Result<Iterator<Item =Kvpair>,KvError>;
}



