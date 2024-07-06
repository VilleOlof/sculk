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
    pub loot_table: Option<Cow<'a, str>>,

    /// A resource location to the loot table that is used to display items in the vault. If not present, the game uses the loot_table field.
    #[serde(borrow)]
    pub override_loot_table_to_display: Option<Cow<'a, str>>,

    /// The range in blocks when the vault should activate. Defaults to 4.
    pub activation_range: Option<i32>,

    /// The range in blocks when the vault should deactivate. Defaults to 4.5.
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
