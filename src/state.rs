use std::sync::{Arc, Mutex};

use color_eyre::Result;

#[derive(Clone, Debug, PartialEq)]
struct SharedState {
    pub count: i32,
    pub amount: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    shared: Arc<Mutex<SharedState>>,
}

impl PartialEq for AppState {
    fn eq(&self, _other: &Self) -> bool {
        // Since we can't easily compare Mutex contents and we're primarily using this
        // for option comparison, we'll just return true for the same type.
        // This is a simplification - in practice you might want a more robust comparison.
        true
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            shared: Arc::new(Mutex::new(SharedState {
                count: 0,
                amount: "1".to_string(),
            })),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_count(&self) -> Result<i32> {
        let state = self.shared.lock().map_err(|_| color_eyre::eyre::eyre!("Failed to lock state"))?;
        Ok(state.count)
    }

    pub fn set_count(&self, count: i32) -> Result<()> {
        let mut state = self.shared.lock().map_err(|_| color_eyre::eyre::eyre!("Failed to lock state"))?;
        state.count = count;
        Ok(())
    }

    pub fn get_amount(&self) -> Result<String> {
        let state = self.shared.lock().map_err(|_| color_eyre::eyre::eyre!("Failed to lock state"))?;
        Ok(state.amount.clone())
    }

    pub fn set_amount(&self, amount: String) -> Result<()> {
        let mut state = self.shared.lock().map_err(|_| color_eyre::eyre::eyre!("Failed to lock state"))?;
        state.amount = amount;
        Ok(())
    }

    pub fn increment_count(&self) -> Result<()> {
        let amount = self.get_amount()?.parse::<i32>().unwrap_or(1);
        let current_count = self.get_count()?;
        self.set_count(current_count.saturating_add(amount))
    }

    pub fn decrement_count(&self) -> Result<()> {
        let amount = self.get_amount()?.parse::<i32>().unwrap_or(1);
        let current_count = self.get_count()?;
        self.set_count(current_count.saturating_sub(amount))
    }

    pub fn increment_amount(&self) -> Result<()> {
        let current_amount = self.get_amount()?.parse::<i32>().unwrap_or(1);
        self.set_amount(current_amount.saturating_add(1).to_string())
    }

    pub fn decrement_amount(&self) -> Result<()> {
        let current_amount = self.get_amount()?.parse::<i32>().unwrap_or(1);
        if current_amount > 1 {
            self.set_amount(current_amount.saturating_sub(1).to_string())
        } else {
            Ok(())
        }
    }
}
