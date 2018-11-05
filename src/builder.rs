use circuit::Circuit;
use gate::Gate;
use aes::enc;
pub struct Builder{
    pub cir : Circuit,
}

impl Builder{
    pub fn new()->Builder{
        Builder{
            cir : Circuit::new(),
        }
    }
    pub fn input(&mut self)->usize{
        let out:Vec<u128>=vec![rand::random::<u64>() as u128,rand::random::<u64>() as u128];
        self.cir.add_gate(Gate::Input{out},vec![])        
    }
    pub fn output(&mut self)->usize{
        let out:Vec<u128>=vec![rand::random::<u64>() as u128,rand::random::<u64>() as u128];
        self.cir.add_gate(Gate::Output{out},vec![])        
    }
    pub fn and(&mut self,x : usize,y :usize) ->usize{
        let out:Vec<u128>=vec![rand::random::<u64>() as u128,rand::random::<u64>() as u128];
        let mut tab:Vec<u128>=vec![];
        
        let mut in_val = [[0_u128,0_u128],[0_u128,0_u128]];
        for i in 0..in_val.len(){
            let id=if i==0 {x} else {y};
            in_val[i]=match &self.cir.gates[id]{
                Gate::Input{out:_} =>{
                    [out[0],out[1]]
                },
                Gate::Output{out:_} =>{
                    [out[0],out[1]]
                },
                Gate::And{tab:_,out:_} =>{
                    [out[0],out[1]]
                },
                Gate::Xor{tab:_,out:_} =>{
                    [out[0],out[1]]
                },
                Gate::Not{tab:_,out:_} =>{
                    [out[0],out[1]]
                },
            };
        }
        for i in 0..in_val[0].len(){
            for j in 0..in_val[1].len(){
                tab.push(enc(in_val[1][j],enc(in_val[0][i],out[(i&j) as usize])));
            }
        }
        for i in 1..tab.len(){
            use rand::Rng;
            let pos = rand::thread_rng().gen_range(0,i);
            tab.swap(i,pos);
        }
        self.cir.add_gate(Gate::And{tab,out},vec![x,y])
    }
}