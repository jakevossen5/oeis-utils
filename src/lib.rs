use lazy_static::lazy_static;
use num_bigint::BigInt;
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

/// Represents the whole OEIS database, which is sequence of [`Series`]
pub struct OEISDatabase {
    series: Vec<Series>,
}

impl OEISDatabase {
    pub fn series(&self) -> &Vec<Series> {
        &self.series
    }

    pub fn from_path(path: &PathBuf) -> Result<Self, std::io::Error> {
        let f = File::open(path);
        match f {
            Ok(f) => {
                let reader = BufReader::new(f);
                let series = reader
                    .lines()
                    .map(|s| s.unwrap())
                    .skip_while(|s| s.starts_with('#'))
                    .map(|s| Series::from_str(&s).unwrap())
                    .collect();

                Ok(Self { series })
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub enum NumberValue {
    InRange(i128),
    OutOfRange(BigInt),
}

/// Represents one series in the OEIS database. The `id` represents the number in "A000055".
#[derive(Debug, Clone, Hash)]
pub struct Series {
    id: usize,
    values: Vec<NumberValue>,
}

impl Series {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn values(&self) -> &Vec<NumberValue> {
        &self.values
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r#"A(?P<Id>\d{6}) (?P<vals>[,\-?\d*,]+),"#).unwrap();
}

impl FromStr for Series {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = RE.captures(s);
        match caps {
            Some(m) => Ok(Self {
                id: m.name("Id").unwrap().as_str().parse().unwrap(),
                values: m
                    .name("vals")
                    .unwrap()
                    .as_str()
                    .split(',')
                    .filter(|s| !s.is_empty())
                    // .map(|s| BigInt::parse_bytes(s.as_bytes(), 10).unwrap())
                    .map(|s| match s.parse::<i128>() {
                        Ok(n) => NumberValue::InRange(n),
                        Err(_) => {
                            NumberValue::OutOfRange(BigInt::parse_bytes(s.as_bytes(), 10).unwrap())
                        }
                    })
                    .collect(),
            }),
            None => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a344199() {
        let text = "A344199 ,18,36,60,252,708,834,900,2178,7722,7980,";
        let s = Series::from_str(text).unwrap();
        assert_eq!(s.id(), 344199);
        assert_eq!(
            *s.values(),
            vec![18, 36, 60, 252, 708, 834, 900, 2178, 7722, 7980]
                .iter()
                .map(|e| NumberValue::InRange(*e))
                .collect::<Vec<NumberValue>>()
        );
    }

    #[test]
    fn a000001() {
        let text = "A000001 ,0,1,1,1,2,1,2,1,5,2,2,1,5,1,2,1,14,1,5,1,5,2,2,1,15,2,2,5,4,1,4,1,51,1,2,1,14,1,2,2,14,1,6,1,4,2,2,1,52,2,5,1,5,1,15,2,13,2,2,1,13,1,2,4,267,1,4,1,5,1,4,1,50,1,2,3,4,1,6,1,52,15,2,1,15,1,2,1,12,1,10,1,4,2,";
        let s = Series::from_str(text).unwrap();
        assert_eq!(s.id(), 1);
        assert_eq!(
            *s.values(),
            vec![
                0, 1, 1, 1, 2, 1, 2, 1, 5, 2, 2, 1, 5, 1, 2, 1, 14, 1, 5, 1, 5, 2, 2, 1, 15, 2, 2,
                5, 4, 1, 4, 1, 51, 1, 2, 1, 14, 1, 2, 2, 14, 1, 6, 1, 4, 2, 2, 1, 52, 2, 5, 1, 5,
                1, 15, 2, 13, 2, 2, 1, 13, 1, 2, 4, 267, 1, 4, 1, 5, 1, 4, 1, 50, 1, 2, 3, 4, 1, 6,
                1, 52, 15, 2, 1, 15, 1, 2, 1, 12, 1, 10, 1, 4, 2
            ]
            .iter()
            .map(|e| NumberValue::InRange(*e))
            .collect::<Vec<NumberValue>>()
        );
    }
}
