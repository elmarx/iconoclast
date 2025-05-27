use crate::TaskId;
use uuid::Uuid;
impl From<Uuid> for TaskId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
