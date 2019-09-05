use chrono::prelude::*;

#[derive(Debug)]
pub struct Person {
    pub age: u32,
    pub name: String,
    pub born_day: Day,
    pub license: DateTime<Utc>,
}
#[derive(Debug)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}
impl Person {
    pub fn get_name(&self) -> &str {
        &self.name[..]
    }

    fn get_license() -> DateTime<Utc> {
        Utc::now()
    }
    pub fn create(age: u32, name: String, born_day: Day) -> Person {
        Person {
            age,
            name,
            born_day,
            license: Person::get_license(),
        }
    }
}
