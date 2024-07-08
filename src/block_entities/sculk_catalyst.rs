use fastnbt::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SculkCatalyst {
    /// List of sculk charge clusters associated with the sculk catalyst.
    cursors: Vec<Cursor>,
}

/// A sculk charge cluster. Each cluster is stored within a single sculk block.

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Cursor {
    /// How many sculk charges are being carried in the cluster.
    pub charge: i32,

    /// List of three integers representing the coordinates of the cluster.
    pub pos: [i32; 3],

    /// 1 or 0 (true/false) - Not to be confused with update_delay. This controls the decay of the cluster. If this is true, then the cluster was spread from a sculk block or sculk vein, and this cluster can only spread to sculk, sculk veins, and blocks with the sculk_replaceable tag set to true. If this is false, then all the sculk charges disappear when the cluster discharges to a block that is not in the sculk family.
    ///
    /// *No clue why this is an i32, wiki says its an int.*  
    pub decay_delay: i32,

    /// Delay in game ticks until the cluster begins to discharge (or travel) after being created. Usually starts at 1 game tick when a cluster discharges to a new block.
    pub update_delay: i32,

    /// If the block to replace is an air or water block, the block is replaced with sculk veins, and the faces where the sculk veins are placed are also stored in their block state. The sculk veins never grow directly on the faces of a sculk block. The same thing is done to any air or water blocks that are adjacent to blocks that are adjacent to this sculk block, if sculk veins can't grow in the blocks adjacent to this sculk block without growing directly on the faces of sculk blocks.
    // TODO: Research what this value is.
    pub facings: Vec<Value>,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "cursors": [
            {
                "charge": 1,
                "pos": [0, 0, 0],
                "decay_delay": 0,
                "update_delay": 1,
                "facings": []
            },
            {
                "charge": 3,
                "pos": [5, 5, 5],
                "decay_delay": 0,
                "update_delay": 1,
                "facings": []
            }
        ]
    });

    let sculk_catalyst: SculkCatalyst = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(sculk_catalyst.cursors.len(), 2);

    let cursor = &sculk_catalyst.cursors[0];

    assert_eq!(cursor.charge, 1);
    assert_eq!(cursor.pos, [0, 0, 0]);
    assert_eq!(cursor.decay_delay, 0);
    assert_eq!(cursor.update_delay, 1);
    assert_eq!(cursor.facings, Vec::<Value>::new());

    let nbt = fastnbt::to_value(&sculk_catalyst).unwrap();

    assert_eq!(nbt, nbt);
}

#[cfg(test)]
#[test]
fn empty_test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "cursors": []
    });

    let sculk_catalyst: SculkCatalyst = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(sculk_catalyst.cursors.len(), 0);

    let nbt = fastnbt::to_value(&sculk_catalyst).unwrap();

    assert_eq!(nbt, nbt);
}
