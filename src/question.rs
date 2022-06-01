struct Cacher< T:Fn(u32)->u32 > {
    cacula:T,
    value:Option<u32>,
}

impl<T> Cacher<T> where T:Fn(u32)->u32{
    fn new(calcu:T) -> Cacher<T> {
        Cacher {
            cacula:calcu,
            value:None,
        }
    }

    // 在此作用范围添加
    // fn cacula(&self,x:u32) -> u32 {
    //      32
    // }
    // 下面可用 let v = self.cacula(arg);
    //

    fn value(&mut self, arg:u32) -> u32 {

        //        在此作用范围添加
        //        fn cacula(x:u32) -> u32 {
        //            32
        //        }
        //        下面可用 let v = cacula(arg);

        match self.value {
            Some(v) => v,
            None => {
                // 疑问: 这里闭包调用为什么要放在一个圆括号内？ self.cacula(arg) 不行吗？
                // help: to call the function stored in `cacula`, surround the field access with parentheses
                // 可能三种调用方式:cacula(arg);  self.cacula(arg);  (self.cacula)(arg)
                // 之所添加“圆括号”是为了区分是“方法调用”还是“闭包调用”
                let v = self.cacula(arg);
                self.value = Some(v);
                v
            }
        }

    }
}