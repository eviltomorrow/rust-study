pub fn synthesize(a: &[super::AminoAcid]) {
    for e in a {
        match e {
            super::AminoAcid::Lys => println!("Lys"),
        }
    }
}
