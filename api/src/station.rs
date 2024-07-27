use serde::Serializer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::str::FromStr;

// MTR Station
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Station {
    pub code: Code,
    pub id: u32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub line_code: Code,
}

// Compare stations against other stations by their code
impl PartialEq for Station {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

// Compare a station against a code
impl PartialEq<Code> for Station {
    fn eq(&self, &other: &Code) -> bool {
        self.code == other
    }
}

// Compare a station against a string (first 3 chars)
impl PartialEq<&str> for Station {
    fn eq(&self, other: &&str) -> bool {
        self.code == *other
    }
}

// A Code, made up of 3 characters.
#[derive(Debug, Deserialize, Copy, Clone, Eq, Hash)]
pub struct Code(char, char, char);

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for Code {
    type Err = ();

    fn from_str(code: &str) -> Result<Self, Self::Err> {
        let code = code.chars().collect::<Vec<char>>();
        return if code.len() != 3 {
            Err(())
        } else {
            Ok(Self(code[0], code[1], code[2]))
        };
    }
}

// simple way to add two codes together
impl Add for Code {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let c1 = self.0 as u32 + other.0 as u32;
        let c2 = self.1 as u32 + other.1 as u32;
        let c3 = self.2 as u32 + other.2 as u32;

        Self(
            std::char::from_u32(c1 % 0x110000).unwrap_or('\0'),
            std::char::from_u32(c2 % 0x110000).unwrap_or('\0'),
            std::char::from_u32(c3 % 0x110000).unwrap_or('\0'),
        )
    }
}

// convert a code to a string (datatype)
impl ToString for Code {
    fn to_string(&self) -> String {
        format!("{}{}{}", self.0, self.1, self.2)
    }
}

// trait to compare codes
impl PartialEq for Code {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

// trait to compare a code against a string (first 3 chars)
impl PartialEq<&str> for Code {
    fn eq(&self, other: &&str) -> bool {
        self.0 == other.chars().nth(0).unwrap()
            && self.1 == other.chars().nth(1).unwrap()
            && self.2 == other.chars().nth(2).unwrap()
    }
}

// Station code 1, Station code 2, Distance in minutes
#[derive(Debug, Deserialize, Serialize, Copy, Clone, Eq)]
pub struct Connection(Code, Code, usize);

// trait to compare connections
impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

// trait to directly compare a code against both codes of the connection
// as it is undirected
impl PartialEq<Code> for Connection {
    fn eq(&self, other: &Code) -> bool {
        self.0 == *other || self.1 == *other
    }
}

// same as above
impl PartialEq<&str> for Connection {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other || self.1 == *other
    }
}

// hash connections by their codes, so we can have a hashset of connections.
impl Hash for Connection {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        hasher.write((self.0 + self.1).to_string().as_bytes());
    }
}
