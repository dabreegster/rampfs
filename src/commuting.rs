use std::collections::HashMap;

use anyhow::Result;
use ex::fs::File;
use serde::Deserialize;

use crate::population::VenueID;
use crate::MSOA;

/// The work activity flows come from a different source, not QUANT data like everything else.
pub fn get_commuting_flows() -> Result<HashMap<MSOA, Vec<(VenueID, f64)>>> {
    for rec in csv::Reader::from_reader(File::open("raw_data/nationaldata/businessRegistry.csv")?)
        .deserialize()
    {
        let _rec: Row = rec?;
    }

    let result = HashMap::new();
    Ok(result)
}

#[allow(unused)]
#[derive(Deserialize)]
struct Row {
    #[serde(rename = "MSOA11CD")]
    msoa: MSOA,
    lng: f64,
    lat: f64,

    // TODO What do all of these mean?
    id: String,
    size: usize,
    // TODO Each person also has this
    sic1d07: usize,
    // sic2d07 isn't unused
}
