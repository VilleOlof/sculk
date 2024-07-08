use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Vault<'a> {
    /// Configuration data that does not automatically change. All fields are optional.
    #[serde(borrow)]
    pub config: VaultConfig<'a>,

    /// Data that is only stored on the server.
    #[serde(borrow)]
    pub server_data: VaultServerData<'a>,

    /// Data that is synced between the server and client.
    #[serde(borrow)]
    pub shared_data: VaultSharedData<'a>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct VaultConfig<'a> {
    ///  A resource location to the loot table that is ejected when unlocking the vault. Defaults to "minecraft:chests/trial_chambers/reward"
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot_table: Option<Cow<'a, str>>,

    /// A resource location to the loot table that is used to display items in the vault. If not present, the game uses the loot_table field.
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_loot_table_to_display: Option<Cow<'a, str>>,

    /// The range in blocks when the vault should activate. Defaults to 4.
    pub activation_range: Option<i32>,

    /// The range in blocks when the vault should deactivate. Defaults to 4.5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deactivation_range: Option<i32>,

    /// The key item that is used to check for valid keys. Defaults to "minecraft:trial_key"
    #[serde(borrow)]
    pub key_item: Item<'a>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct VaultServerData<'a> {
    ///  A set of player UUIDs that have already received their rewards from this vault.
    pub rewarded_plauyers: Vec<i128>,

    /// The game time when the vault processes block state changes, such as changing from unlocking to ejecting after a delay.
    pub state_updating_resumes_at: i64,

    /// List of item stacks that have been rolled by the loot table and are waiting to be ejected.
    #[serde(borrow)]
    pub items_to_eject: Vec<Item<'a>>,

    /// The total amount of item stacks that need to be ejected.
    pub total_ejections_needed: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct VaultSharedData<'a> {
    /// The item that is currently being displayed.
    #[serde(borrow)]
    pub display_item: Item<'a>,

    /// A set of player UUIDs that are within range of the vault.
    pub connected_players: Vec<i128>,

    /// The range in blocks when the vault emits particles.
    pub connected_particles_range: f64,
}

#[cfg(test)]
#[test]
fn test() {
    use fastnbt::nbt;

    let nbt = nbt!({
        "config": {
            "loot_table": "minecraft:chests/trial_chambers/reward",
            "override_loot_table_to_display": "minecraft:chests/trial_chambers/reward",
            "activation_range": 4,
            "deactivation_range": 4,
            "key_item": {
                "Slot": 0u8,
                "id": "minecraft:trial_key",
                "Count": 1
            }
        },
        "server_data": {
            "rewarded_plauyers": [],
            "state_updating_resumes_at": 0i64,
            "items_to_eject": [],
            "total_ejections_needed": 0
        },
        "shared_data": {
            "display_item": {
                "Slot": 0u8,
                "id": "minecraft:stone",
                "Count": 1
            },
            "connected_players": [],
            "connected_particles_range": 0.0f64
        }
    });

    let vault: Vault = fastnbt::from_value(&nbt).unwrap();

    assert_eq!(
        vault.config.loot_table.as_ref().unwrap(),
        "minecraft:chests/trial_chambers/reward"
    );
    assert_eq!(
        vault
            .config
            .override_loot_table_to_display
            .as_ref()
            .unwrap(),
        "minecraft:chests/trial_chambers/reward"
    );
    assert_eq!(vault.config.activation_range.unwrap(), 4);
    assert_eq!(vault.config.deactivation_range.unwrap(), 4);
    assert_eq!(vault.config.key_item.id, "minecraft:trial_key");

    assert_eq!(vault.server_data.rewarded_plauyers, vec![]);
    assert_eq!(vault.server_data.state_updating_resumes_at, 0);
    assert_eq!(vault.server_data.items_to_eject, vec![]);
    assert_eq!(vault.server_data.total_ejections_needed, 0);

    assert_eq!(vault.shared_data.display_item.id, "minecraft:stone");
    assert_eq!(vault.shared_data.connected_players, vec![]);
    assert_eq!(vault.shared_data.connected_particles_range, 0.0);

    let serialized_nbt = fastnbt::to_value(&vault).unwrap();

    assert_eq!(nbt, serialized_nbt);
}
