


pub fn to_bytes(val : u128) -> [u8;16] {
    let mut array = [0_u8;16];
    for i in 0..array.len(){
        array[i]=(val>>(8*i)) as u8;
    }
    array 
}
pub fn has_zeros(x:u128) ->bool{
    let mut flag=true;
    for i in 64..128 {
        if (x>>i&1) as u8 !=0{
            flag=false;
        }
    }
    flag
}

pub fn from_bytes(array : [u8;16]) -> u128{
    let mut val=0_u128;
    for i in 0..array.len(){
        val+=(array[i] as u128)<<(8*i);
    }
    val
}

pub fn enc(key:u128,plain:u128) -> u128{
    let key_bytes=to_bytes(key);
    let plain_bytes=to_bytes(key);
    let mut cipher_bytes=[0_u8;16];
    let aes=crypto::aessafe::AesSafe128Encryptor::new(&key_bytes);
    use crypto::symmetriccipher::BlockEncryptor;
    aes.encrypt_block(&plain_bytes,&mut cipher_bytes);
    let cipher=from_bytes(cipher_bytes);
    cipher
}

pub fn dec(key:u128,cipher:u128) -> u128{
    let key_bytes=to_bytes(key);
    let cipher_bytes=to_bytes(key);
    let mut plain_bytes=[0_u8;16];
    let aes=crypto::aessafe::AesSafe128Decryptor::new(&key_bytes);
    use crypto::symmetriccipher::BlockDecryptor;
    aes.decrypt_block(&cipher_bytes,&mut plain_bytes);
    let plain=from_bytes(plain_bytes);
    plain
}