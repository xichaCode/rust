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

pub trait Persion1: Free {
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

impl <T> prersonl for Customer<PersonalPlan>{
    fn advance_feature(&self) {
        println!("Dear {}(as our valuable customer {}), enjoy this advanced feature!",self.name, self.id );
    }
}

pub struct FreePlan;
pub struct PersonalPlan(f32);

impl <T> Customer<T> {
    pub fn new(name: String) -> Self{
        Self{
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name,
            _type: PhantomData::default(),
        }
    }
}

pub fn subscribe(customer: Customer<FreePlan>, payment: f32) -> Customer<PersonalPlan>{
    let _plan = PersonalPlan(payment);
    customer.into()
}