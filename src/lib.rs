use std::io;

pub struct Input {}
impl Input {
    pub fn read_int() -> u32 {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Unable to read");
        s.trim().parse::<u32>().expect("Failed to parse")
    }

    pub fn read_string() -> String {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Unable to read");
        s.trim().to_string()
    }

    pub fn read_vec() -> Vec<u32> {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Unable to read");
        let mut v: Vec<u32> = Vec::new();
        for i in s.split_whitespace() {
            v.push(i.trim().parse::<u32>().expect("Failed to parse"));
        }
        v
    }
}