//! Database ChEMBL
//! https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/latest/
//! 

use std::io::prelude::*;
use rand::prelude::*;

const CHRLDB_CHEMBL_TXTFILE: &str = "CHRLDB_CHEMBL_TXTFILE";

type ChemblID = String;
type CanonicalSMILES = String;
type StandardInchi = String;
type StandardInchiKey = String;

pub struct EntryChembl {
    pub chembl_id: ChemblID,
    pub smiles: CanonicalSMILES,
    pub inchi: StandardInchi,
    pub inchi_key: StandardInchiKey
}

impl EntryChembl {
    pub fn new(v: Vec<&str>) -> Self {
        let (chembl_id, smiles, inchi, inchi_key) = (String::from(v[0]), String::from(v[1]), String::from(v[2]), String::from(v[3]));
        Self { chembl_id, smiles, inchi, inchi_key }
    }
}

type DataChembl = std::collections::HashMap<String, EntryChembl>;

pub struct SourceChembl {
    filepath: String,
    data: DataChembl 
}

impl SourceChembl {
    pub fn new() -> Self {
        let filepath = std::env::var(CHRLDB_CHEMBL_TXTFILE).expect(format!("{} to notset !", CHRLDB_CHEMBL_TXTFILE).as_str());
        Self { filepath, data: DataChembl::new() }
    }

    fn sanitize(&mut self) {
        self.data.remove("chembl_id");
    }


    pub fn load(&mut self) {
        self.data.clear();

        let file = std::fs::File::open(&self.filepath).unwrap();
        let reader = std::io::BufReader::new(file);
        // let data = reader.lines().take(size)
        self.data = reader.lines()
            .map(|l| {
                    let line = l.unwrap();
                    let v = line.as_str().split('\t').collect::<Vec<&str>>();
                    (String::from(v[0]), EntryChembl::new(v))
                }
            )
            .collect::<Vec<(ChemblID, EntryChembl)>>()
            .into_iter()
            .collect();

        self.sanitize();
    }

    pub fn get(&self, id: &ChemblID) -> Option<&EntryChembl> {
        self.data.get(id)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn choices(&self, size: usize) -> Vec<&EntryChembl> {
        let mut rng = thread_rng();
        let marks: Vec<bool> = (0..self.len())
            .map(|_| rng.gen_range(0..self.len()) <= size )
            .collect();

        self.data.values().enumerate()
            .filter(|(idx, _)| marks[*idx])
            .map(|(_, v)| v)
            .take(size)
            .collect()
    }
}

#[cfg(test)]
mod test_chembl {
    use super::*;

    #[test]
    fn test_source_chembl() {
        let mut sc = SourceChembl::new();
        sc.load();
        assert_eq!(sc.len(), 9999);
        let ec = sc.get(&String::from("CHEMBL503634")).unwrap();
        assert_eq!(ec.smiles, "COc1c(O)cc(O)c(C(=N)Cc2ccc(O)cc2)c1O");    
        assert_eq!(ec.inchi, "InChI=1S/C15H15NO5/c1-21-15-12(19)7-11(18)13(14(15)20)10(16)6-8-2-4-9(17)5-3-8/h2-5,7,16-20H,6H2,1H3");
        assert_eq!(ec.inchi_key, "OPELSESCRGGKAM-UHFFFAOYSA-N");
        let selected = sc.choices(10);
        assert_eq!(selected.len(), 10);
    }
}