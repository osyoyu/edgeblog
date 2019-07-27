use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;

use serde::{Deserialize, Serialize};
use tinysegmenter::tokenize;

type Label = String;

pub struct Document {
    pub label: Label,
    pub content: String,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Index {
    pub inv: HashMap<String, HashSet<Label>>,
}

pub fn generate_index(documents: Vec<Document>) -> Index {
    let mut index = Index {
         inv: HashMap::new(),
    };

    {
        let inv = &mut index.inv;

        for document in documents {
            let tokens = tokenize(&document.content);
            for token in tokens {
                match inv.entry(token.clone()) {
                    Entry::Vacant(e) => {
                        let mut hashset = HashSet::new();
                        hashset.insert(document.label.clone());
                        e.insert(hashset);
                    },
                    Entry::Occupied(mut e) => { e.get_mut().insert(document.label.clone()); },
                }
            }
        }

    }

    let index_str = serde_json::to_string(&index).unwrap();

    index
}

pub fn load_index_from_json(json: String) -> Index {
    serde_json::from_str(&json).expect("failed to load index file")
}
