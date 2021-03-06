use crate::CONFIG;
use std::{collections::HashMap, fs, path::Path};

#[derive(PartialEq, Eq, Hash)]
pub enum FileTypes {
    EgkAllgemein,
    EgkGeschuetzt,
    EgkPersoenlich,
    EgkMFDFHCAEF,
    EgkPruefungsnachweis,
    EgkMFEFGDO,
    EgkResult,
    KvkBinDaten,
    KvkDaten,
}

#[macro_export]
macro_rules! filename_by_type {
    ($xml_type:ident) => {
        FILES.get(&$xml_type).expect("No matching file type found!")
    };
}

impl FileTypes {
    pub fn delete(&self) {
        if self.exists() {
            if let Some(file) = FILES.get(self) {
                fs::remove_file(file)
                    .unwrap_or_else(|_| panic!("Unable to delete {}. Aborting...", file));
                println!("Deleted old {}", file);
            }
        }
    }

    pub fn exists(&self) -> bool {
        let path_from_config = &CONFIG.read().output.path;
        let output_path = Path::new(path_from_config);
        match FILES.get(self) {
            Some(file) => output_path.join(file).as_path().exists(),
            None => false,
        }
    }
}

lazy_static! {
    pub static ref FILES: HashMap<FileTypes, &'static str> = {
        vec![
            (
                FileTypes::EgkAllgemein,
                "eGK_allgemeineVersicherungsdaten.xml",
            ),
            (
                FileTypes::EgkGeschuetzt,
                "eGK_geschuetzteVersichertendaten.xml",
            ),
            (
                FileTypes::EgkPersoenlich,
                "eGK_PersoenlicheVersichertendaten.xml",
            ),
            (FileTypes::EgkMFDFHCAEF, "eGK_MFDF_HCA_EF_StatusVD.xml"),
            (FileTypes::EgkPruefungsnachweis, "eGK_Pruefungsnachweis.xml"),
            (FileTypes::EgkMFEFGDO, "eGK_MFEFGDO.xml"),
            (FileTypes::EgkResult, "Result.xml"),
            (FileTypes::KvkBinDaten, "KVK_Daten.bin"),
            (FileTypes::KvkDaten, "KVK.dat"),
        ]
        .into_iter()
        .collect()
    };
}
