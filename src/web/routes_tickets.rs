use crate::ctx::Ctx;
use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(tfc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    let result = mc.create_ticket(tfc, ctx.user_id()).await?; // doet Chone het ook met unwrap?
    Ok(Json(result))
}

async fn list_tickets(State(mc): State<ModelController>, _ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    let result = mc.list_tickets().await?;
    Ok(Json(result))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
    _ctx: Ctx,
) -> Result<Json<Ticket>> {
    let result = mc.delete_ticket(id).await?;
    Ok(Json(result))
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}
