//! Database ChEMBL
//! https://ftp.ebi.ac.uk/pub/databases/chembl/ChEMBLdb/latest/
//! 
//! # How to use
//!
//! ``` 
//! use chiral_db_sources::chembl::SourceChembl;
//! 
//! let filepath = std::path::Path::new("./data/chembl_30_chemreps_100.txt");
//! let sc = SourceChembl::new(&filepath);
//! assert_eq!(sc.len(), 100);
//! let ec = sc.get(&String::from("CHEMBL503634")).unwrap();
//! assert_eq!(ec.smiles, "COc1c(O)cc(O)c(C(=N)Cc2ccc(O)cc2)c1O");    
//! assert_eq!(ec.inchi, "InChI=1S/C15H15NO5/c1-21-15-12(19)7-11(18)13(14(15)20)10(16)6-8-2-4-9(17)5-3-8/h2-5,7,16-20H,6H2,1H3");
//! assert_eq!(ec.inchi_key, "OPELSESCRGGKAM-UHFFFAOYSA-N");
//! 
//! let data_all = sc.get_all();
//! assert_eq!(data_all.keys().count(), 100);
//! 
//! let selected = sc.choices(10);
//! assert_eq!(selected.len(), 10);
//! ```

use std::io::prelude::*;
use rand::prelude::*;

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
    data: DataChembl 
}

impl SourceChembl {
    pub fn new(filepath: &std::path::Path) -> Self {
        let mut sc = Self { data: DataChembl::new() };
        sc.load(filepath);
        sc
    }

    fn sanitize(&mut self) {
        self.data.remove("chembl_id");
    }

    pub fn load(&mut self, filepath: &std::path::Path) {
        self.data.clear();

        let file = std::fs::File::open(filepath).unwrap();
        let reader = std::io::BufReader::new(file);
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

    pub fn get_all(&self) -> &DataChembl {
        &self.data
    }

    pub fn get_smiles_id_pairs(&self) -> (Vec<&String>, Vec<String>) {
        (
            self.data.values()
                .map(|ec| &ec.smiles)
                .collect(),
            self.data.keys()
                .map(|id| id.clone())
                .collect()
        )
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn choices(&self, size: usize) -> Vec<&EntryChembl> {
        let mut rng = thread_rng();
        let marks: Vec<bool> = (0..self.len())
            .map(|_| rng.gen_range(0..self.len()) <= size * 2 )
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
        let filepath = std::path::Path::new("./data/chembl_30_chemreps_100.txt");
        let sc = SourceChembl::new(&filepath);
        assert_eq!(sc.len(), 100);
        let ec = sc.get(&String::from("CHEMBL503634")).unwrap();
        assert_eq!(ec.smiles, "COc1c(O)cc(O)c(C(=N)Cc2ccc(O)cc2)c1O");    
        assert_eq!(ec.inchi, "InChI=1S/C15H15NO5/c1-21-15-12(19)7-11(18)13(14(15)20)10(16)6-8-2-4-9(17)5-3-8/h2-5,7,16-20H,6H2,1H3");
        assert_eq!(ec.inchi_key, "OPELSESCRGGKAM-UHFFFAOYSA-N");

        let data_all = sc.get_all();
        assert_eq!(data_all.keys().count(), 100);

        let selected = sc.choices(10);
        assert_eq!(selected.len(), 10);
    }
}