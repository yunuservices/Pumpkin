pub mod player_change_world;
pub mod player_chat;
pub mod command_preprocess;
pub mod player_command_send;
pub mod player_custom_payload;
pub mod drop_item;
pub mod edit_book;
pub mod egg_throw;
pub mod exp_change;
pub mod fish;
pub mod interact_at_entity;
pub mod interact_entity;
pub mod item_break;
pub mod item_consume;
pub mod item_damage;
pub mod item_held;
pub mod item_mend;
pub mod player_gamemode_change;
pub mod player_interact_entity_event;
pub mod player_interact_event;
pub mod player_interact_unknown_entity_event;
pub mod player_join;
pub mod kick;
pub mod player_leave;
pub mod level_change;
pub mod player_login;
pub mod pre_login;
pub mod advancement_done;
pub mod animation;
pub mod armor_stand_manipulate;
pub mod bed_enter;
pub mod bed_leave;
pub mod bucket_empty;
pub mod bucket_fill;
pub mod bucket_entity;
pub mod changed_main_hand;
pub mod register_channel;
pub mod unregister_channel;
pub mod player_move;
pub mod resource_pack_status;
pub mod respawn;
pub mod pickup_arrow;
pub mod portal;
pub mod recipe_discover;
pub mod riptide;
pub mod shear_entity;
pub mod spawn_location;
pub mod statistic_increment;
pub mod velocity;
pub mod harvest_block;
pub mod swap_hand_items;
pub mod player_teleport;
pub mod toggle_flight;
pub mod toggle_sneak;
pub mod toggle_sprint;

use std::sync::Arc;

use crate::entity::player::Player;

/// A trait representing events related to players.
///
/// This trait provides a method to retrieve the player associated with the event.
pub trait PlayerEvent: Send + Sync {
    /// Retrieves a reference to the player associated with the event.
    ///
    /// # Returns
    /// A reference to the `Arc<Player>` involved in the event.
    fn get_player(&self) -> &Arc<Player>;
}
