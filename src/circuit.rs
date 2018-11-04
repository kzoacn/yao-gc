use gate::Gate;
use wire::Wire;
pub struct circuit{
    gates : Vec<Gate>,
    fan_in : Vec<Vec<usize> >,
    fan_out : Vec<Option<Wire> >,
}

impl circuit{
    fn eval(&self) -> Vec<(usize,Wire)>{
        vec![]
    }
    fn dec(&self,c : Vec<(usize,Wire)>) -> Vec<(usize,u8)>{
        let mut ans:Vec<(usize,u8)>=vec![];
        for (i,w) in &c{
            let out=&self.gates[*i];
            if let Gate::Output{out} = *out{

            }
            
            return vec![];
        }
        vec![]
    }
}