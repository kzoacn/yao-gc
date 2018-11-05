use gate::Gate;
use wire::Wire;
use std::collections::VecDeque;
#[derive(Debug)]
pub struct Circuit{
    pub gates : Vec<Gate>,
    pub fan_in : Vec<Vec<usize> >,
}

impl Circuit{
    pub fn new()->Circuit{
        Circuit{
            gates : vec![],
            fan_in : vec![],
        }
    }

    pub fn add_gate(&mut self,gate :Gate,fin : Vec<usize>) -> usize{
        let out = self.gates.len();
        self.fan_in.push(Vec::new());
        self.gates.push(gate);
        self.fan_in[out]=fin;
        out
    }


    pub fn eval(&self,input : Vec<(usize,Wire)>) -> Vec<Option<Wire> >{
        let mut ans : Vec<(usize,Wire)>=vec![];
        let mut output : Vec<Option<Wire> >=vec![];
        let mut fan_out : Vec<Vec<usize> >=vec![];
        let mut deg : Vec<usize> =vec![];
        fan_out.resize(self.gates.len(), Vec::new());
        deg.resize(self.gates.len(),0);
        output.resize(self.gates.len(),None);

        for i in 0..self.fan_in.len(){
            deg[i]=self.fan_in[i].len();
            for j in 0..self.fan_in[i].len(){
                fan_out[self.fan_in[i][j]].push(i);
            }
        }
        let fan_out=fan_out;
        let mut q : VecDeque<usize>=VecDeque::new();
        for (i,w) in &input {
            output[*i]=Some(*w);
            q.push_back(*i);
        }

        while !q.is_empty(){
            let u=*q.front().unwrap();
            q.pop_front();

            if let None=output[u]{
                let mut wires:Vec<Wire>=vec![];
                for i in &self.fan_in[u]{
                    wires.push(output[*i].unwrap());
                }
                output[u]=Some(Gate::eval(&self.gates[u],wires));
            }

            for v in &fan_out[u]{
                deg[*v]-=1;
                if deg[*v]==0{
                    
                    q.push_back(*v);
                }
            }
        }
        
        output
    }
    pub fn dec(&self,c : Vec<(usize,Wire)>) -> Vec<(usize,u8)>{
        let mut ans:Vec<(usize,u8)>=vec![];
        for (i,w) in &c{
            if self.gates[*i].get_output(0).val==w.val {
                ans.push((*i,0));
            }else{
                ans.push((*i,1));
            }
        }
        ans
    }
}