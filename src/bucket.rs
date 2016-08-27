use std::collections::{BTreeMap, HashMap};
use std::f64;

use futures::{BoxFuture, Future};
use json;
use yaml_rust;
use time::{Duration, Timespec, Tm, at_utc, now_utc};

use database::{Database, Error};

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

pub struct BucketState {
    content: u64,
    last_drip: Tm,
}

impl BucketState {
    pub fn new(content: u64, last_drip: Tm) -> BucketState {
        BucketState {
            content: content,
            last_drip: last_drip,
        }
    }

    pub fn try_new(state: Vec<u8>) -> Option<BucketState> {
        String::from_utf8(state).ok().and_then(|s| {
            json::parse(&*s).ok().map(|j| {
                BucketState::new(
                    j["content"].as_u64().unwrap_or(0),
                    j["lastDrip"].as_i64().map(|x| at_utc(Timespec::new(x, 0))).unwrap_or(now_utc())
                )
            })
        })
    }
}

#[derive(Clone)]
pub struct Bucket {
    name: String,
    interval: u64,
    per_interval: u64,
    purpose: Option<String>,
    size: u64,
    until: Option<Tm>,
}

impl Bucket {
    pub fn new(name: String, interval: u64, per_interval: u64, size: u64) -> Bucket {
        Bucket {
            name: name,
            interval: interval,
            per_interval: per_interval,
            purpose: None,
            size: size,
            until: None,
        }
    }

    fn get_content(&self, state: &BucketState) -> i32 {
        if self.per_interval == 0 || self.interval == 0 {
            return 0;
        }
        let now = now_utc();
        let delta = now - state.last_drip;
        let drip = f64::floor((delta.num_milliseconds() as f64 * (self.per_interval as f64 / self.interval as f64))) as u64;
        let content = (state.content + drip) as u64;
        if content > self.size { self.size as i32 } else { content as i32 }
    }

    fn get_duration_to_completion(&self, state: &BucketState) -> Duration {
        if self.per_interval == 0 || self.interval == 0 {
            return Duration::zero();
        }

        let missing = self.size - state.content;
        let ms_to_completion = f64::ceil(missing as f64 * self.interval as f64 / self.per_interval as f64) as i64;
        Duration::milliseconds(ms_to_completion)
    }

    fn get_reset_time(&self, state: &BucketState) -> Tm {
        let now = now_utc();
        now + self.get_duration_to_completion(state)
    }

    fn get_key_status(&self, name: Vec<u8>, state: Vec<u8>) -> Option<(String, i32, i32, i32)> {
        String::from_utf8(name).ok().and_then(|name| {
            BucketState::try_new(state).map(|state| {
                let content = self.get_content(&state);
                let reset = self.get_reset_time(&state);
                (name, content, reset.to_timespec().sec as i32, 0)
            })
        })
    }
    pub fn status(&self, key: &str, db: &Database) -> BoxFuture<Vec<(String, i32, i32, i32)>, Error> {
        let bucket = self.clone();
        db.list(key.as_bytes()).map(move |r| {
            r.into_iter().flat_map(|el| bucket.get_key_status(el.0, el.1)).collect()
        }).boxed()
    }
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
    pub fn add(&mut self, bucket: Bucket) {
        let name = bucket.name.clone();
        self.buckets.insert(name, bucket);
    }

    pub fn get(&self, name: &str) -> Option<&Bucket> {
        self.buckets.get(name)
    }

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

    fn parse_until(&mut self, h: &BTreeMap<yaml_rust::Yaml, yaml_rust::Yaml>) -> Result<Option<Tm>, BucketError> {
        match h.get(&yaml_rust::Yaml::from_str("until")) {
            Some(s) => s.as_i64().ok_or(BucketError::WrongType).map(|x| Some(at_utc(Timespec::new(x, 0)))),
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
