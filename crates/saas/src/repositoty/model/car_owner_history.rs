use surrealdb::{Datetime, RecordId};

#[derive(Debug, Clone)]
pub struct CarOwnerHistory {
    id: RecordId,
    car: RecordId,
    client: RecordId,
    created_at: Datetime,
}
