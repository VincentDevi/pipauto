use derive_more::Constructor;

#[derive(Debug, Clone, Constructor)]
pub struct Email(String);
