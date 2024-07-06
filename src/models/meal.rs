use openmensa_rust_interface::Meal;
use tabled::Tabled;

/// Represents a meal formatted for tabular display.
#[derive(Tabled)]
pub struct TabledMeal {
    /// The name of the meal.
    pub name: String,
    /// The price of the meal for students.
    pub student_price: f64,
    /// The price of the meal for employees.
    pub employee_price: f64,
    /// The price of the meal for guests.
    pub guest_price: f64,
    /// Additional notes or information about the meal.
    pub notes: String,
}

/// Converts a Meal into a TabledMeal for tabular representation.
impl From<Meal> for TabledMeal {
    /// Converts a Meal struct into a TabledMeal struct.
    ///
    /// # Arguments
    ///
    /// * `meal` - The Meal struct to convert.
    ///
    /// # Returns
    ///
    /// A TabledMeal struct with data converted from the Meal.
    fn from(meal: Meal) -> Self {
        TabledMeal {
            name: meal.name,
            student_price: meal.prices.students.unwrap_or(0.0),
            employee_price: meal.prices.employees.unwrap_or(0.0),
            guest_price: meal.prices.pupils.unwrap_or(0.0),
            notes: meal.notes.join(", "),
        }
    }
}
