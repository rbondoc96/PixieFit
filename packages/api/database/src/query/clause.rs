use super::bind::SqlxBindable;

pub struct WhereClause<'w> {
    pub column: &'static str,
    pub operator: &'static str,
    pub value: Box<dyn SqlxBindable + 'w + Send + Sync>,
}
