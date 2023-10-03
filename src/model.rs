use std::sync::{Arc, Mutex};

use crate::Result;
use serde::Serialize;

#[derive(Serialize, Clone)]
struct Ticket {
    id: u64,
    title: String,
}

struct TicketForCreate {
    title: String,
}

#[derive(Clone)]
struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    async fn new() -> Result<Self> {
        Ok(ModelController {
            tickets_store: Arc::default(),
        })
    }

    async fn create_ticket(&self, to_create: TicketForCreate) -> Result<Ticket> {
        // eventuele error is van ander type dan eigen error type...
        let mut lock_result = self.tickets_store.lock().unwrap();
        // mutexguard implementeert dereferencing, dus zou wel
        let safe_id = lock_result.len() as u64;
        let ticket = Ticket {
            id: safe_id,
            title: to_create.title,
        };
        let clone = ticket.clone();
        lock_result.push(Some(ticket));
        Ok(clone)
    }

    async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let lock_result = self.tickets_store.lock().unwrap();
        Ok(lock_result
            .iter()
            .filter_map(|o| o.as_ref().map(|ticket| ticket.clone()))
            .collect())
    }
}
