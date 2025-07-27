use serde::Serialize;

use crate::game::items::{Item, ItemData, ItemQuantity};
use crate::types::TickDuration;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    pub name: String,
    #[serde(flatten)]
    pub data: ItemData,
    cooldown: TickDuration,

    remaining_ticks: TickDuration,
    quantity: ItemQuantity,
}

impl InventoryItem {
    pub fn new(item: &Item) -> Self {
        InventoryItem {
            name: item.name.to_string(),
            data: item.data,
            cooldown: item.cooldown,
            quantity: item.quantity,
            remaining_ticks: 0,
        }
    }

    pub fn quantity(&self) -> &ItemQuantity {
        &self.quantity
    }

    pub fn pick_up(&mut self, qtty: &ItemQuantity) {
        match qtty {
            ItemQuantity::Infinite => self.quantity = ItemQuantity::Infinite,
            ItemQuantity::Finite(qtty) => {
                if let ItemQuantity::Finite(ref mut existing_qtty) = self.quantity {
                    *existing_qtty += qtty;
                }
            }
        }
    }

    pub fn is_usable(&self, ignore_cooldowns: bool) -> bool {
        if !ignore_cooldowns && self.remaining_ticks > 0 {
            false
        } else {
            match self.quantity {
                ItemQuantity::Infinite => true,
                ItemQuantity::Finite(quantity) => quantity > 0,
            }
        }
    }

    pub fn use_one(&mut self) {
        self.remaining_ticks = self.cooldown + 1;
        if let ItemQuantity::Finite(ref mut quantity) = self.quantity {
            *quantity = quantity.saturating_sub(1);
        }
    }

    pub fn tick(&mut self) {
        self.remaining_ticks = self.remaining_ticks.saturating_sub(1);
    }
}
