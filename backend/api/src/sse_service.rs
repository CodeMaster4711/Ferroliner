use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct SseEvent {
    pub kind: String,
    pub payload: serde_json::Value,
    pub user_ids: Vec<Uuid>,
    pub org_id: Uuid,
}
