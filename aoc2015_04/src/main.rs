const FIVE_ZEROS_MASK: u128 = u128::from_ne_bytes([0xFF, 0xFF, 0xF0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
const SIX_ZEROS_MASK: u128 = u128::from_ne_bytes([0xFF, 0xFF, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

#[allow(dead_code)]
fn brute_force(secret: &str, prefix: &str) -> u32 
{
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{}{}", secret, i));
        let hex = format!("{:x}", digest);
        if hex.starts_with(prefix) {
            return i;
        }
        i += 1;
    }
}

fn brute_force_optim(secret: &str, mask: &u128) -> u32
{
    let mut i = 0;    
    loop {
        let i_string = i.to_string();
        let mut data = String::with_capacity(secret.len() + i_string.len());
        data.push_str(secret);
        data.push_str(i_string.as_str());

        let digest = md5::compute(data);
        let dg = u128::from_ne_bytes(digest.0);
        
        if dg & mask == 0 {
            return i;
        }
        i += 1;
    }
}

fn main() {
    let secret = "yzbqklnj";

    let silver = brute_force_optim(secret, &FIVE_ZEROS_MASK);
    println!("Silver = {}", silver);
    
    let gold = brute_force_optim(secret, &SIX_ZEROS_MASK);
    println!("Gold = {}", gold);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brute_force1() {
        assert_eq!(brute_force("abcdef", "00000"), 609043);
    }

    #[test]
    fn test_brute_force2() {
        assert_eq!(brute_force("pqrstuv", "00000"), 1048970);
    }

    #[test]
    fn test_brute_force_optim1() {
        assert_eq!(brute_force_optim("abcdef", &FIVE_ZEROS_MASK), 609043);
    }

    #[test]
    fn test_brute_force_optim2() {
        assert_eq!(brute_force_optim("pqrstuv", &FIVE_ZEROS_MASK), 1048970);
    }
}