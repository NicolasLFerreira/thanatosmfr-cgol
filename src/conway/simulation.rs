use crate::types::cell_configuration::CellConfiguration;
use crate::types::cell_coord::CellCoord;
use std::collections::HashMap;

pub fn simulation(cconf: &CellConfiguration) -> CellConfiguration {
    let mut neighbours: HashMap<CellCoord, u8> = HashMap::new();

    // init alive cells with 0 neighbours
    for coord in cconf.iter() {
        neighbours.insert(coord, 0);
    }

    // compute neighbours
    for ccoord in cconf.iter() {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let d_ccoord = CellCoord::new(dx, dy);

                let key = ccoord + d_ccoord;
                let entry = neighbours.entry(key).or_insert(0);
                *entry += 1;
            }
        }
    }

    let mut new_cconf = CellConfiguration::new();

    // apply survival rules
    for (coord, count) in neighbours {
        if cconf.is_alive(coord) && count == 2 {
            new_cconf.spawn(coord);
        }
        if count == 3 {
            new_cconf.spawn(coord);
        }
    }

    new_cconf
}
