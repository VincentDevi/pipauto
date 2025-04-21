use surrealdb::RecordId;

pub trait Records {
    fn table(&self) -> String;

    fn id(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct ClientRecordId {
    table: String,
    id: String,
}

impl Records for ClientRecordId {
    fn table(&self) -> String {
        self.table.clone()
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

impl TryFrom<RecordId> for ClientRecordId {
    type Error = String;
    fn try_from(value: RecordId) -> Result<Self, Self::Error> {
        let table = value.table().to_string();
        let id = value.key().to_string();
        if table.is_empty() {
            return Err("Table name is empty".to_string());
        }
        if id.is_empty() {
            return Err("ID is empty".to_string());
        }

        // Create a new ClientRecordId with the extracted values
        Ok(ClientRecordId { table, id })
    }
}

impl From<ClientRecordId> for RecordId {
    fn from(value: ClientRecordId) -> Self {
        Self::from_table_key(value.table, value.id)
    }
}
