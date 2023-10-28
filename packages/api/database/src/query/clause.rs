use super::bind::SqlxBindable;

pub struct WhereClause<'w> {
    pub column: &'static str,
    pub operator: &'static str,
    pub value: Box<dyn SqlxBindable + 'w + Send + Sync>,
}

pub enum OrderClause {
    Ascending(&'static str),
    Descending(&'static str),
}

impl ToString for OrderClause {
    fn to_string(&self) -> String {
        match *self {
            Self::Ascending(expr) => format!("{} ASC", expr),
            Self::Descending(expr) => format!("{} DESC", expr),
        }
    }
}
