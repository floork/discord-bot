use mensa_cli_backend::Meal;
use tabled::Tabled;

#[derive(Tabled)]
pub struct TabledMeal {
    // pub id: u64,
    pub name: String,
    // category: String,
    pub student_price: f64,
    pub employee_price: f64,
    pub guest_price: f64,
    pub notes: String,
}

impl From<Meal> for TabledMeal {
    fn from(meal: Meal) -> Self {
        TabledMeal {
            // id: meal.id,
            name: meal.name,
            // category: meal.category,
            student_price: meal.prices.students.unwrap_or(0.0),
            employee_price: meal.prices.employees.unwrap_or(0.0),
            guest_price: meal.prices.pupils.unwrap_or(0.0),
            notes: meal.notes.join(", "),
        }
    }
}
