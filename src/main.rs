use std::collections::HashMap;
use std::mem;

fn main() {
    let f = |x: f64, t: f64| -0.1 * x + 0.0*t;
    let x: EulerMethod = EulerMethod::new(f,0.0, 10.0, 1.0, 100000);
    {
        //let tmp = x.get(0.5123213);
        //println!("{}", tmp)
    }
    for(key, vals) in &x.answer{
        println!("y({}) = {}", key.to_float(),vals);
    }
}

fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

#[derive(Hash, Eq, PartialEq)]
struct Results {
    mes_triplet: (u64, i16, i8),
}
#[allow(dead_code)]
impl Results {
    fn new(num: f64)->Results{
        Results{mes_triplet: integer_decode(num)}
    }
    fn to_float(&self)->f64{
        (self.mes_triplet.0 as f64) * (self.mes_triplet.2 as f64) * (2f64.powf(self.mes_triplet.1 as f64) as f64)
    }
}

#[allow(dead_code)]
struct EulerMethod {
    steps: i32,
    t_initial: f64,
    t_final: f64,
    y_initial: f64,
    answer: HashMap<Results, f64>,
}
#[allow(dead_code)]
impl EulerMethod{
    fn new<F>(func: F, t_0: f64, t_f: f64, y_0: f64, n_steps: i32)->EulerMethod
        where F: Fn(f64, f64)->f64{
            let steps = n_steps as f64;
            let step_size: f64 = (t_f-t_0)/(steps);
            let mut y_of_t = HashMap::new();
            let mut y_n: f64 =  y_0;
            y_of_t.insert(Results::new(t_0) , y_0);
            for i in 0..n_steps{
                let t_n: f64 = t_0 + step_size * (i as f64);
                y_n = y_n + step_size * func(y_n, t_n);
                y_of_t.insert(Results::new(t_n), y_n);
            }
        EulerMethod{ t_initial: t_0, t_final: t_f, y_initial: y_0, steps: n_steps, answer:y_of_t}
    }
    fn get(&mut self, key: f64)->&mut f64{
        //let newkey = Results::new(key);
        self.answer.get_mut(&Results::new(key)).expect("no entry for key")
    }
}
