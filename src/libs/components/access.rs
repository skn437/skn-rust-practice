use super::access_type::{AuthorizationStatus, Database, Employee, KeyCard, ProtectedLocation};

pub fn authorize(name: &str, location: ProtectedLocation) -> Result<AuthorizationStatus, String> {
  let db: Database = Database::connect()?;

  let employee: Employee = db.find_employee(name)?;

  let keycard_level: KeyCard = db.get_keycard_level(&employee)?;

  if keycard_level.get_access_level() >= location.required_access_level() {
    return Ok(AuthorizationStatus::Access);
  }

  Ok(AuthorizationStatus::Deny)
}
