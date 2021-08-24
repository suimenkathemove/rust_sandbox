struct Cacher<C, V>
where
    C: Fn(V) -> V,
    V: Copy,
{
    calculation: C,
    value: Option<V>,
}

impl<C, V> Cacher<C, V>
where
    C: Fn(V) -> V,
    V: Copy,
{
    fn new(calculation: C) -> Self {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: V) -> V {
        match self.value {
            Some(v) => v,
            None => {
                println!("calculate");

                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn main() {
    let mut c = Cacher::new(|a| a);

    let _v = c.value(1);
    let v = c.value(1);

    assert_eq!(v, 1);
}
