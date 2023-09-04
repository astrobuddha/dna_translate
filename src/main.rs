use std::collections::HashMap;

fn main() {
    // println!("Hello, world!");

    let ghd = "AUGGAUCAGACAUAUUCUCUGGAGUCAUUCCUCAACCAUGUCCAAAAGCGCGACCCGAAUCAAACCGAGUUCGCGCAAGCCGUUCGUGAAGUAAUGACCACACUCUGGCCUUUUCUUGAACAAAAUCCAAAAUAUCGCCAGAUGUCAUUACUGGAGCGUCUGGUUGAACCGGAGCGCGUGAUCCAGUUUCGCGUGGUAUGGGUUGAUGAUCGCAACCAGAUACAGGUCAACCGUGCAUGGCGUGUGCAGUUCAGCUCUGCCAUCGGCCCGUACAAAGGCGGUAUGCGCUUCCAUCCGUCAGUUAACCUUUCCAUUCUCAAAUUCCUCGGCUUUGAACAAACCUUCAAAAAUGCCCUGACUACUCUGCCGAUGGGCGGUGGUAAAGGCGGCAGCGAUUUCGAUCCGAAAGGAAAAAGCGAAGGUGAAGUGAUGCGUUUUUGCCAGGCGCUGAUGACUGAACUGUAUCGCCACCUGGGCGCGGAUACCGACGUUCCGGCAGGUGAUAUCGGGGUUGGUGGUCGUGAAGUCGGCUUUAUGGCGGGGAUGAUGAAAAAGCUCUCCAACAAUACCGCCUGCGUCUUCACCGGUAAGGGCCUUUCAUUUGGCGGCAGUCUUAUUCGCCCGGAAGCUACCGGCUACGGUCUGGUUUAUUUCACAGAAGCAAUGCUAAAACGCCACGGUAUGGGUUUUGAAGGGAUGCGCGUUUCCGUUUCUGGCUCCGGCAACGUCGCCCAGUACGCUAUCGAAAAAGCGAUGGAAUUUGGUGCUCGUGUGAUCACUGCGUCAGACUCCAGCGGCACUGUAGUUGAUGAAAGCGGAUUCACGAAAGAGAAACUGGCACGUCUUAUCGAAAUCAAAGCCAGCCGCGAUGGUCGAGUGGCAGAUUACGCCAAAGAAUUUGGUCUGGUCUAUCUCGAAGGCCAACAGCCGUGGUCUCUACCGGUUGAUAUCGCCCUGCCUUGCGCCACCCAGAAUGAACUGGAUGUUGACGCCGCGCAUCAGCUUAUCGCUAAUGGCGUUAAAGCCGUCGCCGAAGGGGCAAAUAUGCCGACCACCAUCGAAGCGACUGAACUGUUCCAGCAGGCAGGCGUACUAUUUGCACCGGGUAAAGCGGCUAAUGCUGGUGGCGUCGCUACAUCGGGCCUGGAAAUGGCACAAAACGCUGCGCGCCUGGGCUGGAAAGCCGAGAAAGUUGACGCACGUUUGCAUCACAUCAUGCUGGAUAUCCACCAUGCCUGUGUUGAGCAUGGUGGUGAAGGUGAGCAAACCAACUACGUGCAGGGCGCGAACAUUGCCGGUUUUGUGAAGGUUGCCGAUGCGAUGCUGGCGCAGGGUGUGAUUUAA";

    // todo move to own translator class
    let mut map = HashMap::new();

    map.insert("AUG", 'M');
    map.insert("GCG", 'A');
    map.insert("UCA", 'S');
    map.insert("GAA", 'E');
    map.insert("GGG", 'G');
    map.insert("GGU", 'G');
    map.insert("AAA", 'K');
    map.insert("GAG", 'E');
    map.insert("AAU", 'N');
    map.insert("CUA", 'L');
    map.insert("CAU", 'H');
    map.insert("UCG", 'S');
    map.insert("UAG", '0'); // stop
    map.insert("GUG", 'V');
    map.insert("UAU", 'Y');
    map.insert("CCU", 'P');
    map.insert("ACU", 'T');
    map.insert("UCC", 'S');
    map.insert("CAG", 'Q');
    map.insert("CCA", 'P');
    map.insert("UAA", '0'); // stop
    map.insert("AGA", 'R');
    map.insert("ACG", 'T');
    map.insert("CAA", 'Q');
    map.insert("UGU", 'C');
    map.insert("GCU", 'A');
    map.insert("UUC", 'F');
    map.insert("AGU", 'S');
    map.insert("AUA", 'I');
    map.insert("UUA", 'L');
    map.insert("CCG", 'P');
    map.insert("AUC", 'I');
    map.insert("UUU", 'F');
    map.insert("CGU", 'R');
    map.insert("UGA", '0'); // stop
    map.insert("GUA", 'V');
    map.insert("UCU", 'S');
    map.insert("CAC", 'H');
    map.insert("GUU", 'V');
    map.insert("GAU", 'D');
    map.insert("CGA", 'R');
    map.insert("GGA", 'G');
    map.insert("GUC", 'V');
    map.insert("GGC", 'G');
    map.insert("UGC", 'C');
    map.insert("CUG", 'L');
    map.insert("CUC", 'L');
    map.insert("CGC", 'R');
    map.insert("CGG", 'R');
    map.insert("AAC", 'N');
    map.insert("GCC", 'A');
    map.insert("AUU", 'I');
    map.insert("AGG", 'R');
    map.insert("GAC", 'D');
    map.insert("ACC", 'T');
    map.insert("AGC", 'S');
    map.insert("UAC", 'Y');
    map.insert("ACA", 'T');
    map.insert("AAG", 'K');
    map.insert("GCA", 'A');
    map.insert("UUG", 'L');
    map.insert("CCC", 'P');
    map.insert("CUU", 'L');
    map.insert("UGG", 'W');

    let codons = get_codons(&ghd.chars().collect());

    for codon in codons.iter() {
        println!("{}", translate(codon.as_str(), &map).to_string().as_str());
    }
}

fn translate(codon: &str, map: &HashMap<&str, char>) -> char {
    if codon.len() != 3 {
        panic!("Invalid codon");
    }

    map.get(codon).unwrap().clone() // todo fix unwrap
}

fn get_codons(seq: &Vec<char>) -> Vec<String> {
    let mut codons = Vec::new();

    let mut i = 0;

    while i + 2 < seq.len() {
        codons.push(seq[i].to_string() + &seq[i + 1].to_string() + &seq[i + 2].to_string());

        i += 3;
    }

    codons
}
