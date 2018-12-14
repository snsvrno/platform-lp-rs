use std::fmt;
use std::marker::PhantomData;

use serde;
use Platform;

struct PlatformVisitor {
    marker: PhantomData<fn() -> Platform>
}

impl PlatformVisitor {
    fn new() -> Self {
        PlatformVisitor {
            marker : PhantomData
        }
    }
}

impl <'de>serde::de::Visitor<'de> for PlatformVisitor {
    type Value = Platform;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("platform")
    }
    
    fn visit_str<A>(self, string:&str) -> Result<Self::Value, A> {
        Ok(Platform::new(string))
    }
}

impl serde::Serialize for Platform {
    fn serialize<S>(&self,serializer : S) -> Result<S::Ok, S::Error> where S : serde::Serializer {
        serializer.serialize_str(&self.to_short_string())
    }
}

// I THINK THIS IS MY ISSUE
impl <'de> serde::Deserialize<'de> for Platform {
    fn deserialize<D>(deserializer : D) -> Result<Platform, D::Error> where D : serde::Deserializer<'de> {
        deserializer.deserialize_str(PlatformVisitor::new())
    }
}
