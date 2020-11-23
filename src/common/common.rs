#[derive(Clone, Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String
}



struct CacherExample<T>
where
    T: Fn(u32) -> u32,
{
    callback: T,
    value: Option<u32>
}

impl <T> CacherExample<T> 
where
    T: Fn(u32) -> u32
{
    fn new(cacher_example:T) -> CacherExample<T> {
        CacherExample {
            callback: cacher_example,
            value:None,
        }
    }
    fn value(&mut self, arg: u32) -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.callback)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}