use std::ops::Index;
use std::str::FromStr;

use std::error::Error;

#[derive(Debug, Default)]
pub struct NF_Z10_011 {
    pub lines: [String; 7],
}

impl Index<u8> for NF_Z10_011 {
    type Output = str;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0..=7 => self.lines[index as usize - 1].as_ref(),
            _ => unreachable!(),
        }
    }
}

impl FromStr for NF_Z10_011 {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = NF_Z10_011::default();

        let lines: Vec<String> = s.lines().map(|x| x.to_string()).collect();

        if lines.len() > 7 {
            return Err("Input must contain no more than 7 lines".into());
        }

        for (i, line) in lines.into_iter().enumerate() {
            res.lines[i] = line;
        }

        Ok(res)
    }
}
