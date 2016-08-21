use std::collections::{BTreeMap, HashMap};

use yaml_rust;
use time;

#[derive(Debug)]
pub enum BucketError {
    WrongType,
    YamlError(yaml_rust::ScanError),
}

impl From<yaml_rust::ScanError> for BucketError {
    fn from(e: yaml_rust::ScanError) -> Self {
        BucketError::YamlError(e)
    }
}

pub struct Bucket {
    name: String,
    interval: u64,
    per_interval: u64,
    purpose: Option<String>,
    size: u64,
    until: Option<time::Tm>,
}

#[derive(Default)]
pub struct Buckets {
    buckets: HashMap<String, Bucket>,
}

macro_rules! parse_interval {
    ($key: expr, $h: expr, $interval: expr) => {
        match $h.get(&yaml_rust::Yaml::from_str($key)) {
            Some(s) => return s.as_i64().ok_or(BucketError::WrongType).map(|x| ($interval, $interval * (x as u64))),
            _ => (),
        };
    }
}

impl Buckets {
    fn parse_size(&mut self, h: &BTreeMap<yaml_rust::Yaml, yaml_rust::Yaml>) -> Result<u64, BucketError> {
        match h.get(&yaml_rust::Yaml::from_str("size")) {
            Some(s) => s.as_i64().ok_or(BucketError::WrongType).map(|x| x as u64),
            _ => Ok(0),
        }
    }

    fn parse_purpose(&mut self, h: &BTreeMap<yaml_rust::Yaml, yaml_rust::Yaml>) -> Result<Option<String>, BucketError> {
        match h.get(&yaml_rust::Yaml::from_str("purpose")) {
            Some(s) => s.as_str().ok_or(BucketError::WrongType).map(|x| Some(x.to_owned())),
            _ => Ok(None),
        }
    }

    fn parse_until(&mut self, h: &BTreeMap<yaml_rust::Yaml, yaml_rust::Yaml>) -> Result<Option<time::Tm>, BucketError> {
        match h.get(&yaml_rust::Yaml::from_str("until")) {
            Some(s) => s.as_i64().ok_or(BucketError::WrongType).map(|x| Some(time::at_utc(time::Timespec::new(x, 0)))),
            _ => Ok(None),
        }
    }

    fn parse_interval(&mut self, h: &BTreeMap<yaml_rust::Yaml, yaml_rust::Yaml>) -> Result<(u64, u64), BucketError> {
        let mut interval = 1000;
        parse_interval!("per_second", h, interval);
        interval *= 60;
        parse_interval!("per_minute", h, interval);
        interval *= 60;
        parse_interval!("per_hour", h, interval);
        interval *= 24;
        parse_interval!("per_day", h, interval);
        Ok((0, 0))
    }

    fn parse_yaml(&mut self, y: yaml_rust::Yaml) -> Result<(), BucketError> {
        for (k, v) in try!(y.as_hash().ok_or(BucketError::WrongType)) {
            let key = try!(k.as_str().ok_or(BucketError::WrongType));
            let h = try!(v.as_hash().ok_or(BucketError::WrongType));
            let (interval, per_interval) = try!(self.parse_interval(h));
            let size = try!(self.parse_size(h));
            let purpose = try!(self.parse_purpose(h));
            let until = try!(self.parse_until(h));
            self.buckets.insert(key.to_owned(), Bucket {
                name: key.to_owned(),
                interval: interval,
                per_interval: per_interval,
                purpose: purpose,
                size: size,
                until: until,
            });
        }
        Ok(())
    }

    fn parse_yamls(&mut self, yamls: Vec<yaml_rust::Yaml>) -> Result<(), BucketError> {
        for y in yamls {
            try!(self.parse_yaml(y))
        }
        Ok(())
    }

    pub fn new(yaml: &str) -> Result<Self, BucketError> {
        let yamls = try!(yaml_rust::yaml::YamlLoader::load_from_str(yaml));
        let mut buckets = Buckets::default();
        try!(buckets.parse_yamls(yamls));
        Ok(buckets)
    }
}

#[cfg(test)]
mod test {
    use time;
    use super::Buckets;

    #[test]
    fn bucket_config() {
        let s = "
user:
    size: 5
    per_second: 10
    purpose: abc
    until: 1471823420
api:
    size: 100
    per_hour: 100
    purpose: def
    until: 1471823430
opt:
    size: 0
        ";

        let b = Buckets::new(s).unwrap();
        assert_eq!(b.buckets.get("user").unwrap().name, "user");
        assert_eq!(b.buckets.get("user").unwrap().size, 5);
        assert_eq!(b.buckets.get("user").unwrap().interval, 1000);
        assert_eq!(b.buckets.get("user").unwrap().per_interval, 10000);
        assert_eq!(b.buckets.get("user").unwrap().purpose, Some("abc".to_owned()));
        assert_eq!(b.buckets.get("user").unwrap().until, Some(time::Tm { tm_sec: 20, tm_min: 50, tm_hour: 23, tm_mday: 21, tm_mon: 7, tm_year: 116, tm_wday: 0, tm_yday: 233, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 }));

        assert_eq!(b.buckets.get("api").unwrap().name, "api");
        assert_eq!(b.buckets.get("api").unwrap().size, 100);
        assert_eq!(b.buckets.get("api").unwrap().interval, 3600000);
        assert_eq!(b.buckets.get("api").unwrap().per_interval, 360000000);
        assert_eq!(b.buckets.get("api").unwrap().purpose, Some("def".to_owned()));
        assert_eq!(b.buckets.get("api").unwrap().until, Some(time::Tm { tm_sec: 30, tm_min: 50, tm_hour: 23, tm_mday: 21, tm_mon: 7, tm_year: 116, tm_wday: 0, tm_yday: 233, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 }));

        assert_eq!(b.buckets.get("opt").unwrap().name, "opt");
        assert_eq!(b.buckets.get("opt").unwrap().size, 0);
        assert_eq!(b.buckets.get("opt").unwrap().interval, 0);
        assert_eq!(b.buckets.get("opt").unwrap().per_interval, 0);
        assert_eq!(b.buckets.get("opt").unwrap().purpose, None);
        assert_eq!(b.buckets.get("opt").unwrap().until, None);
    }
}
