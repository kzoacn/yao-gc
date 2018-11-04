 
use wire::Wire;

use aes::enc;
use aes::dec;
use aes::has_zeros;

pub enum Gate{
    Input {out:Vec<u128>},
    Output {out:Vec<u128>},
    And {tab:Vec<u128>,out:Vec<u128>}   ,
    Xor {tab:Vec<u128>,out:Vec<u128>}   ,
    Not {tab:Vec<u128>,out:Vec<u128>}   ,
}

pub fn eval(gate : Gate,wires : Vec<Wire>) -> Wire{
    
    match gate{
        Gate::Input{out:_} =>{
            assert_eq!(wires.len(),1);
            let ans=Wire::init(wires[0].val);
            ans
        },
        Gate::Output{out:_} =>{
            assert_eq!(wires.len(),1);
            let ans=Wire::init(wires[0].val);
            ans
        },
        Gate::And{tab,out:_}  =>{
            assert_eq!(wires.len(),2);
            let mut ans=Wire::new();
            for c in &tab{
                let plain=dec(wires[1].val,dec(wires[0].val,*c));
                if has_zeros(plain) {
                    ans=Wire::init(plain);
                }
            }
            ans
        },
        Gate::Xor{tab,out:_} =>{
            assert_eq!(wires.len(),2);
            let mut ans=Wire::new();
            for c in &tab{
                let plain=dec(wires[1].val,dec(wires[0].val,*c));
                if has_zeros(plain) {
                    ans=Wire::init(plain);
                }
            }
            ans
        },
        Gate::Not{tab,out:_} =>{
            assert_eq!(wires.len(),1);
            let mut ans=Wire::new();
            for c in &tab{
                let plain=dec(wires[1].val,dec(wires[0].val,*c));
                if has_zeros(plain) {
                    ans=Wire::init(plain);
                }
            }
            ans
        },
    }
}


 