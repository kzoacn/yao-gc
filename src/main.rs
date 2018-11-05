extern crate crypto;
extern crate rand;
mod wire;
mod gate;
mod aes;
mod circuit;
mod builder;
use wire::Wire;
use builder::Builder;
use circuit::Circuit;

fn main() {
    let mut builder = Builder::new();
    let a_in = builder.input();
    let b_in = builder.input();
    let and = builder.and(a_in, b_in);
    let cir = builder.cir;

    println!("{:?}",cir);

    let input = vec![(a_in,cir.gates[a_in].get_output(0)),(b_in,cir.gates[b_in].get_output(0))];

    println!("{:?}",input);

    let wires=cir.eval(input);
    let mut out : Vec<(usize,Wire)>=vec![];
    match wires[and]{
        Some(w) =>{
            out.push( (and,w));
        },
        None =>{
            panic!("boom!");
        },
    };
    println!("{:?}",out);
    println!("{:?}",cir.dec(out));
}
