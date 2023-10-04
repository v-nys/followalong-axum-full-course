use std::sync::{Arc, Mutex};

use crate::{Error, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Clone)]
pub struct Ticket {
    id: u64,
    title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    title: String,
}

#[derive(Clone, Default)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(ModelController {
            tickets_store: Arc::default(),
        })
    }

    pub async fn create_ticket(&self, to_create: TicketForCreate) -> Result<Ticket> {
        // eventuele error is van ander type dan eigen error type...
        let mut guard = self.tickets_store.lock().unwrap();
        // mutexguard implementeert dereferencing, dus zou wel
        let safe_id = guard.len() as u64;
        let ticket = Ticket {
            id: safe_id,
            title: to_create.title,
        };
        let clone = ticket.clone();
        guard.push(Some(ticket));
        Ok(clone)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let guard = self.tickets_store.lock().unwrap();
        Ok(guard
            .iter()
            .filter_map(|o| o.as_ref().map(|ticket| ticket.clone()))
            .collect())
    }

    pub async fn delete_ticket(&self, id: u64) -> Result<Ticket> {
        let mut guard = self.tickets_store.lock().unwrap();
        // note: this works because index position = id in this implementation!
        let ticket = guard.get_mut(id as usize).and_then(|t| t.take());
        ticket.ok_or(Error::TicketDeleteFailIdNotFound { id })
    }
}
