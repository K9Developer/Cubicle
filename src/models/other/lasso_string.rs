use lasso::{Rodeo, Spur};
use std::sync::{OnceLock, RwLock};
use std::fmt::{Display, Formatter};

static INTERNER: OnceLock<RwLock<Rodeo<Spur>>> = OnceLock::new();

fn interner() -> &'static RwLock<Rodeo<Spur>> {
    INTERNER.get_or_init(|| RwLock::new(Rodeo::new()))
}

pub fn intern(s: &str) -> Spur {
    interner().write().unwrap().get_or_intern(s)
}

pub fn resolve(sym: Spur) -> &'static str {
    let guard = interner().read().unwrap();
    // rodeo lives as long as the program so this is fine
    unsafe { std::mem::transmute::<&str, &'static str>(guard.resolve(&sym)) }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct LassoString {
    key: Spur
}

impl LassoString {
    pub fn new(str: &str) -> Self {
        LassoString { key: intern(str) }
    }

    pub fn new_key(key: Spur) -> Self {
        LassoString { key }
    }

    pub fn get(&self) -> &str { resolve(self.key) }
    pub fn to_string(&self) -> String { resolve(self.key).to_string() }
    pub fn key(&self) -> &Spur { &self.key }
}

impl From<Spur> for LassoString {
    fn from(value: Spur) -> Self { LassoString::new_key(value) }
}

impl From<String> for LassoString {
    fn from(value: String) -> Self { LassoString::new_key(intern(value.as_str())) }
}

impl From<&str> for LassoString {
    fn from(value: &str) -> Self { LassoString::new_key(intern(value)) }
}

impl Into<String> for LassoString {
    fn into(self) -> String { self.to_string() }
}

impl Into<&str> for LassoString {
    fn into(self) -> &'static str { resolve(self.key) }
}

impl Into<Spur> for LassoString {
    fn into(self) -> Spur { self.key }
}

impl Display for LassoString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get())
    }
}

