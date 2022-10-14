use maplit::btreeset;
use std::collections::BTreeSet;

pub fn get_blacklist() -> BTreeSet<String> {
    let s = btreeset! {
        "takeaki_fuck",
        "1463_0224",
        "334_Reporter",
        "7Kc1L297eruhZaH",
        "BIZARREFLIFE",
        "FelsenBurg1967",
        "IKASARU2"
    };
    s.iter().map(|s| s.to_string()).collect()
}
