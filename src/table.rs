//! Layout engine for phonological inventory tables.

use std::collections::{BTreeSet, HashMap, HashSet};

use crate::{
    feature::{
        Airstream, ConsonantFeature, Depth, Feature, FricativeKind, Height, Manner, Modifier,
        Place, PlaceCategory, RearArticulation, Voice,
    },
    Phoneme,
};

pub enum Heading {
    Feature(Feature),
    Category(PlaceCategory),
}

impl Heading {
    fn try_merge_with(&self, other: &Heading) -> Option<(Heading, f32)> {
        use ConsonantFeature::Place;
        use Feature::Consonant;
        match (self, other) {
            (Heading::Feature(Consonant(Place(p1))), Heading::Feature(Consonant(Place(p2)))) => {
                if p1.category() == p2.category() {
                    Some((Heading::Category(p1.category()), 0.0))
                } else {
                    None
                }
            }
            (Heading::Feature(Consonant(Place(place))), Heading::Category(cat))
            | (Heading::Category(cat), Heading::Feature(Consonant(Place(place)))) => {
                if place.category() == *cat {
                    Some((Heading::Category(*cat), 0.0))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

pub struct Cell {
    pub phonemes: Vec<Phoneme>,
}

pub enum Row {
    Group { heading: Heading, rows: Vec<Row> },
    Individual { heading: Heading, cells: Vec<Cell> },
}

impl Row {
    fn heading(&self) -> &Heading {
        match self {
            Row::Group { heading, .. } => heading,
            Row::Individual { heading, .. } => heading,
        }
    }
}

pub enum Column {
    Group {
        heading: Heading,
        columns: Vec<Column>,
    },
    Individual {
        heading: Heading,
    },
}

impl Column {
    fn heading(&self) -> &Heading {
        match self {
            Column::Group { heading, .. } => heading,
            Column::Individual { heading } => heading,
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
                use ConsonantFeature::{DoubleArticulation, Manner, Place};
                use Feature::Consonant;
                let (manner, place) = p.features().iter().fold((None, None), |(m, p), f| match f {
                    manner @ Consonant(Manner(..)) => (Some(manner), p),
                    place @ (Consonant(Place(..)) | Consonant(DoubleArticulation(..))) => {
                        (m, Some(place))
                    }
                    _ => (m, p),
                });

                if let (Some(manner), Some(place)) = (manner, place) {
                    Some((p, manner, place))
                } else {
                    None
                }
            })
            .fold(
                (Vec::new(), BTreeSet::new(), BTreeSet::new()),
                |(mut ps, mut ms, mut ps_), (p, m, p_)| {
                    ps.push(p.clone());
                    ms.insert(m);
                    ps_.insert(p_);
                    (ps, ms, ps_)
                },
            );

        todo!()
    }
}
