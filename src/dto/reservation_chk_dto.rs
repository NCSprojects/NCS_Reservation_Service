use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct ReservationLimits {
    pub total_adults: Option<i32>,
    pub total_children: Option<i32>,
}
