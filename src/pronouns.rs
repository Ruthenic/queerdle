use std::{fmt::Display, mem::variant_count};

use rand::{distributions::Standard, prelude::Distribution};

#[derive(PartialEq, PartialOrd, Debug)]
pub enum BasePronouns {
    HE,
    SHE,
    THEY,
    IT,
    XE,
    VI,
    ANY,
}

impl Distribution<BasePronouns> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BasePronouns {
        match rng.gen_range(0..variant_count::<BasePronouns>()) {
            0 => BasePronouns::HE,
            1 => BasePronouns::SHE,
            2 => BasePronouns::THEY,
            3 => BasePronouns::IT,
            4 => BasePronouns::XE,
            5 => BasePronouns::VI,
            _ => BasePronouns::ANY,
        }
    }
}

impl BasePronouns {
    pub fn to_str(&self, position: u8) -> &str {
        if position == 0 {
            match self {
                BasePronouns::HE => "he",
                BasePronouns::SHE => "she",
                BasePronouns::THEY => "they",
                BasePronouns::IT => "it",
                BasePronouns::XE => "xe",
                BasePronouns::VI => "vi",
                BasePronouns::ANY => "any",
            }
        } else {
            match self {
                BasePronouns::HE => "him",
                BasePronouns::SHE => "her",
                BasePronouns::THEY => "them",
                BasePronouns::IT => "its",
                BasePronouns::XE => "xem",
                BasePronouns::VI => "vim",
                BasePronouns::ANY => "all",
            }
        }
    }
}

pub struct Pronouns {
    pub first: BasePronouns,
    pub second: BasePronouns,
}

impl Distribution<Pronouns> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Pronouns {
        let first = rng.gen::<BasePronouns>();
        let second = rng.gen::<BasePronouns>();
        Pronouns { first, second }
    }
}

impl Display for Pronouns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}",
            self.first.to_str(0),
            if self.first == self.second {
                self.second.to_str(1)
            } else {
                self.second.to_str(0)
            }
        )
    }
}

impl From<String> for Pronouns {
    fn from(value: String) -> Self {
        let binding = value.to_ascii_uppercase();
        let mut split = binding.split('/');
        let first = match split.next() {
            Some(v) => match v {
                "HE" => BasePronouns::HE,
                "SHE" => BasePronouns::SHE,
                "THEY" => BasePronouns::THEY,
                "IT" => BasePronouns::IT,
                "XE" => BasePronouns::XE,
                "VI" => BasePronouns::VI,
                _ => BasePronouns::ANY,
            },
            None => BasePronouns::ANY,
        };
        let second = match split.next() {
            Some(v) => match v {
                "HE" | "HIM" => BasePronouns::HE,
                "SHE" | "HER" => BasePronouns::SHE,
                "THEY" | "THEM" => BasePronouns::THEY,
                "IT" | "ITS" => BasePronouns::IT,
                "XE" | "XEM" => BasePronouns::XE,
                "VI" | "VIM" => BasePronouns::VI,
                _ => BasePronouns::ANY,
            },
            None => BasePronouns::ANY,
        };

        Pronouns { first, second }
    }
}
