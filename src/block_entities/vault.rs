use crate::{
    error::SculkParseError,
    item::Item,
    traits::FromCompoundNbt,
    util::{get_owned_optional_string, get_t_compound_vec},
    uuid::Uuid,
};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vault {
    /// Configuration data that does not automatically change. All fields are optional.
    pub config: VaultConfig,

    /// Data that is only stored on the server.
    pub server_data: VaultServerData,

    /// Data that is synced between the server and client.
    pub shared_data: VaultSharedData,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VaultConfig {
    ///  A resource location to the loot table that is ejected when unlocking the vault. Defaults to "minecraft:chests/trial_chambers/reward"
    pub loot_table: Option<String>,

    /// A resource location to the loot table that is used to display items in the vault. If not present, the game uses the loot_table field.
    pub override_loot_table_to_display: Option<String>,

    /// The range in blocks when the vault should activate. Defaults to 4.
    pub activation_range: Option<i32>,

    /// The range in blocks when the vault should deactivate. Defaults to 4.5.
    pub deactivation_range: Option<i32>,

    /// The key item that is used to check for valid keys. Defaults to "minecraft:trial_key"
    pub key_item: Item,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VaultServerData {
    ///  A set of player UUIDs that have already received their rewards from this vault.
    pub rewarded_players: Vec<Uuid>,

    /// The game time when the vault processes block state changes, such as changing from unlocking to ejecting after a delay.
    pub state_updating_resumes_at: i64,

    /// List of item stacks that have been rolled by the loot table and are waiting to be ejected.
    pub items_to_eject: Vec<Item>,

    /// The total amount of item stacks that need to be ejected.
    pub total_ejections_needed: i32,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VaultSharedData {
    /// The item that is currently being displayed.
    pub display_item: Item,

    /// A set of player UUIDs that are within range of the vault.
    pub connected_players: Vec<Uuid>,

    /// The range in blocks when the vault emits particles.
    pub connected_particles_range: f64,
}

impl FromCompoundNbt for Vault {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let config = if let Some(nbt) = nbt.compound("config") {
            VaultConfig::from_compound_nbt(&nbt)?
        } else {
            return Err(SculkParseError::MissingField("config".into()));
        };

        let server_data = if let Some(nbt) = nbt.compound("server_data") {
            VaultServerData::from_compound_nbt(&nbt)?
        } else {
            return Err(SculkParseError::MissingField("server_data".into()));
        };

        let shared_data = if let Some(nbt) = nbt.compound("shared_data") {
            VaultSharedData::from_compound_nbt(&nbt)?
        } else {
            return Err(SculkParseError::MissingField("shared_data".into()));
        };

        Ok(Vault {
            config,
            server_data,
            shared_data,
        })
    }
}

impl FromCompoundNbt for VaultConfig {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let loot_table = get_owned_optional_string(&nbt, "loot_table");

        let override_loot_table_to_display =
            get_owned_optional_string(&nbt, "override_loot_table_to_display");

        let activation_range = nbt.int("activation_range");
        let deactivation_range = nbt.int("deactivation_range");

        let key_item = if let Some(nbt) = nbt.compound("key_item") {
            Item::from_compound_nbt(&nbt)?
        } else {
            return Err(SculkParseError::MissingField("key_item".into()));
        };

        Ok(VaultConfig {
            loot_table,
            override_loot_table_to_display,
            activation_range,
            deactivation_range,
            key_item,
        })
    }
}

impl FromCompoundNbt for VaultServerData {
    fn from_compound_nbt(nbt: &simdnbt::borrow::NbtCompound) -> Result<Self, SculkParseError>
    where
        Self: Sized,
    {
        let rewarded_players = Uuid::from_nbt_to_vec(&nbt, "rewarded_players");

        let state_updating_resumes_at =
            nbt.long("state_updating_resumes_at")
                .ok_or(SculkParseError::MissingField(
                    "state_updating_resumes_at".into(),
                ))?;

        let items_to_eject = get_t_compound_vec(&nbt, "items_to_eject", Item::from_compound_nbt)?;

        let total_ejections_needed =
            nbt.int("total_ejections_needed")
                .ok_or(SculkParseError::MissingField(
                    "total_ejections_needed".into(),
                ))?;

        Ok(VaultServerData {
            rewarded_players,
            state_updating_resumes_at,
            items_to_eject,
            total_ejections_needed,
        })
    }
}

impl FromCompoundNbt for VaultSharedData {
    fn from_compound_nbt(
        nbt: &simdnbt::borrow::NbtCompound,
    ) -> Result<Self, crate::error::SculkParseError>
    where
        Self: Sized,
    {
        let display_item = nbt
            .compound("display_item")
            .map(|nbt| Item::from_compound_nbt(&nbt))
            .ok_or(SculkParseError::MissingField("display_item".into()))??;

        let connected_players = Uuid::from_nbt_to_vec(&nbt, "connected_players");

        let connected_particles_range =
            nbt.double("connected_particles_range")
                .ok_or(SculkParseError::MissingField(
                    "connected_particles_range".into(),
                ))?;

        Ok(VaultSharedData {
            display_item,
            connected_players,
            connected_particles_range,
        })
    }
}
