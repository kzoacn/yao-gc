 
use wire::Wire;

use aes::enc;
use aes::dec;
use aes::has_zeros;

pub enum Gate{
    And {tab:Vec<u128>,out:Vec<u128>}   ,
    Xor {tab:Vec<u128>,out:Vec<u128>}   ,
    Not {tab:Vec<u128>,out:Vec<u128>}   ,
}

pub fn eval(gate : Gate,wires : Vec<Wire>) -> Wire{
    assert_eq!(wires.len(),2);
    match gate{
        Gate::And{tab,out}  =>{
            let mut ans=Wire::new();
            for c in &tab{
                let plain=dec(wires[1].val,dec(wires[0].val,*c));
                if has_zeros(plain) {
                    ans=Wire::init(plain);
                }
            }
            ans
        },
        Gate::Xor{tab,out} =>{
            let mut ans=Wire::new();
            for c in &tab{
                let plain=dec(wires[1].val,dec(wires[0].val,*c));
                if has_zeros(plain) {
                    ans=Wire::init(plain);
                }
            }
            ans
        },
        Gate::Not{tab,out} =>{
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


 