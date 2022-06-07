use std::{
    marker::PhantomData,
    sync::atomic::{
        AtomicU64,Ordering
    },
};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub struct Customer<T> {
    id: u64,
    name: String,
    _type: PhantomData<T>,
}

pub trait Free {
    fn feature1(&self);
    fn feature2(&self);
}

pub trait Personal : Free {
    fn advance_feature(&self);
}

impl<T> Free for Customer<T>{
    fn feature1(&self) { 
        println!("feature1 fro {}", self.name);
    }

    fn feature2(&self) {
        println!("feature2 fro {}", self.name);
    }
}

impl Personal for Customer<PersonalPlan>{
    fn advance_feature(&self) {
        println!("Dear {}(as our valuable customer {}), enjoy this advanced feature!",self.name, self.id );
    }
}

pub struct FreePlan;
pub struct PersonalPlan(f32);

impl <T> Customer<T> {
    //构造函数，返回Customer的一个实例
    pub fn new(name: String) -> Self{
        Self{
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

impl From<Customer<FreePlan>> for Customer<PersonalPlan> {
    fn from(plan: Customer<FreePlan>) -> Self {
        Self::new(plan.name)
    }
}

impl Personal for Customer<FreePlan> {
    fn advance_feature(&self){
        println!("hejian ++++++++++++advance_feature,{}",self.name);
    }
}

pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan>{
    let _plan = PersonalPlan(payment);
    customer.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_customer(){
        //创建Customer 实例，其中new相当于java的构造函数, 实例为customer的结构体, 结构体类型为FreePlan
        //结构体实体实现了Free tarit ，其中有两个方法，feature1, feature2
        let customer = Customer::<FreePlan>::new("hejina".into()); 
        //通过customer实例，访问结构体中的方法 feature1 和 feature2
        customer.feature1();
        customer.feature2();
        //泛型FreePlan的customer实例，实现PersonalPlan trait，即advance_feature 
        customer.advance_feature();
        //创建泛型为PersonlPlan的customer 实例，其中new相当于java的构造函数, 实例为Customer的结构体, 结构体类型为PersonlPlan
        //PersonalPlan是一个结构构类型，实现了Personal tarit
        let customer_ = Customer::<PersonalPlan>::new("hejian_".into());
        customer_.advance_feature();
    
        let customer = subscribe(customer, 6.99);
        customer.feature1();
        customer.feature2();
        customer.advance_feature();
    }
}