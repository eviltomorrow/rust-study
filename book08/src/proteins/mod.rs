pub enum AminoAcid {
    Lys,
}

pub mod synthesis;

use synthesis::synthesize;

pub fn say() {
    let mut v: Vec<AminoAcid> = Vec::new();
    v.push(AminoAcid::Lys);
    synthesize(&v[..]);
}
