use std::str::FromStr;

#[derive(Debug)]
pub enum SemVerToken {
    Some(u16),
    Any,
}

#[derive(Debug)]
pub struct SemVer {
    pub major: SemVerToken,
    pub minor: SemVerToken,
    pub patch: SemVerToken,
}

impl FromStr for SemVerToken {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Any),
            _ => match s.parse::<u16>() {
                Ok(v) => Ok(Self::Some(v)),
                _ => Err(format!("{} is invalid", s)),
            },
        }
    }
}

impl SemVerToken {
    pub fn value(&self) -> Option<u16> {
        match self {
            Self::Any => None,
            Self::Some(v) => Some(*v),
        }
    }
}

impl ToString for SemVerToken {
    fn to_string(&self) -> String {
        match self {
            Self::Any => "*".to_string(),
            Self::Some(v) => v.to_string(),
        }
    }
}


impl FromStr for SemVer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // note: this only works since there's only ASCII char
        if let Some(p1) = s.find('.') {
            if let Some(p2) = s[p1 + 1..].find('.') {
                let major = s[0..p1].parse::<SemVerToken>()?;
                let minor = s[p1 + 1..p1 + 1 + p2].parse::<SemVerToken>()?;
                let patch = s[p1 + 2 + p2..].parse::<SemVerToken>()?;
                return Ok(SemVer {
                    major,
                    minor,
                    patch,
                });
            }
        }

        Err(format!("{} invalid semver", s))

    }
}


impl SemVerToken {
    fn is_match(&self, t: &SemVerToken) -> bool {
        match self {
            SemVerToken::Any => true,
            SemVerToken::Some(left) => match t {
                SemVerToken::Some(right) => left == right,
                _ => false,
            },
        }
    }
}

impl SemVer {
    pub fn any() -> Self {
        Self {
            major: SemVerToken::Any,
            minor: SemVerToken::Any,
            patch: SemVerToken::Any
        }
    }

    pub fn any_minor(major: u16) -> Self {
        Self {
            major: SemVerToken::Some(major),
            minor: SemVerToken::Any,
            patch: SemVerToken::Any
        }
    }
    pub fn any_patch(major: u16, minor: u16) -> Self {
        Self {
            major: SemVerToken::Some(major),
            minor: SemVerToken::Some(minor),
            patch: SemVerToken::Any
        }
    }

    pub fn is_match(&self, s: &str) -> bool {
        match s.parse::<SemVer>() {
            Err(_) => false,
            Ok(other) => self.is_match_other(&other),
        }
    }

    pub fn is_match_other(&self, other: &Self) -> bool {
        self.major.is_match(&other.major)
            && self.minor.is_match(&other.minor)
            && self.patch.is_match(&other.patch)
    }
}

impl ToString for SemVer {
    fn to_string(&self) -> String {
        format!(
            "{}.{}.{}",
            self.major.to_string(),
            self.minor.to_string(),
            self.patch.to_string()
        )
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    // use ::test::Bencher;

    #[test]
    fn test_parse_semver() {
        let v = SemVer::from_str("1.0.5").unwrap();
        assert_eq!(v.major.value().unwrap(), 1);
        assert_eq!(v.minor.value().unwrap(), 0);
        assert_eq!(v.patch.value().unwrap(), 5);
        assert_eq!(v.to_string(), "1.0.5");

        let v = SemVer::from_str("1.0.*").unwrap();
        assert_eq!(v.major.value().unwrap(), 1);
        assert_eq!(v.minor.value().unwrap(), 0);
        assert!(v.patch.value().is_none());
        assert_eq!(v.to_string(), "1.0.*");

        let v = SemVer::from_str("54.*.1235").unwrap();
        assert_eq!(v.major.value().unwrap(), 54);
        assert!(v.minor.value().is_none());
        assert_eq!(v.patch.value().unwrap(), 1235);
        assert_eq!(v.to_string(), "54.*.1235");
    }

    #[test]
    fn test_match_semver() {
        let v = SemVer::from_str("1.0.5").unwrap();
        assert!(v.is_match("1.0.5"));
        assert!(!v.is_match("1.0.*"));
        assert!(!v.is_match("1.0.42"));
        assert!(!v.is_match("1.1.5"));
        assert!(!v.is_match("2.0.5"));

        let v = SemVer::from_str("1.0.*").unwrap();
        assert!(v.is_match("1.0.5"));
        assert!(v.is_match("1.0.*"));
        assert!(v.is_match("1.0.42"));
        assert!(!v.is_match("1.1.5"));
        assert!(!v.is_match("2.0.5"));

        let v = SemVer::from_str("1.*.*").unwrap();
        assert!(v.is_match("1.0.5"));
        assert!(v.is_match("1.0.42"));
        assert!(v.is_match("1.1.5"));
        assert!(!v.is_match("2.0.5"));

        let v = SemVer::from_str("*.*.*").unwrap();
        assert!(v.is_match("1.0.5"));
        assert!(v.is_match("1.0.42"));
        assert!(v.is_match("1.1.5"));
        assert!(v.is_match("2.0.5"));
    }
}
