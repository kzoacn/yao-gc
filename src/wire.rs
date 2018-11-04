


pub struct Wire{
    pub val : u128,
}

impl Wire{
    pub fn new() -> Wire{
        Wire{
            val : 0_u128,
        }
    }
    pub fn init(v : u128) -> Wire{
        Wire{
            val : v,
        }
    }

    pub fn rand(&mut self){
        self.val=(rand::random::<u64>() as u128) + ((rand::random::<u64>() as u128)<<64);
    }

}