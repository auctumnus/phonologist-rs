//! Layout engine for phonological inventory tables.

use std::collections::{BTreeSet, HashMap, HashSet};

use crate::{
    Phoneme, feature::{
        Airstream, ConsonantFeature, Depth, Feature, FricativeKind, Height, Manner, Modifier, Place, RearArticulation, Voice
    }
};

pub struct Cell {
    pub phonemes: Vec<Phoneme>,
}

pub enum Row {
    Group {
        feature: Feature,
        rows: Vec<Row>,
    },
    Individual {
        feature: Feature,
        cells: Vec<Cell>,
    }
}

impl Row {
    fn feature(&self) -> &Feature {
        match self {
            Row::Group { feature, .. } => feature,
            Row::Individual { feature, .. } => feature,
        }
    }
}

pub enum Column {
    Group {
        feature: Feature,
        columns: Vec<Column>,
    },
    Individual {
        feature: Feature,
    }
}

impl Column {
    fn feature(&self) -> &Feature {
        match self {
            Column::Group { feature, .. } => feature,
            Column::Individual { feature } => feature,
        }
    }
}

pub struct Table {
    pub rows: Vec<Row>,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn consonants(phonemes: Vec<Phoneme>) -> Table {
        let (phonemes, manners, places) = phonemes
            .iter()
            .filter_map(|p| {
                let (manner, place) = p.features().iter().fold((None, None), |(m, p), f| {
                    match f {
                        Feature::Consonant(ConsonantFeature::Manner()) => (Some(manner), p),
                        Feature::Place(place) => (m, Some(place)),
                        _ => (m, p),
                    }
                });

                if let (Some(manner), Some(place)) = (manner, place) {
                    Some((p, manner, place))
                } else {
                    None
                }
            })
            .fold((Vec::new(), BTreeSet::new(), BTreeSet::new()), |(mut ps, mut ms, mut ps_), (p, m, p_)| {
                ps.push(p.clone());
                ms.insert(m);
                ps_.insert(p_);
                (ps, ms, ps_)
            });

        todo!()
    }
}

