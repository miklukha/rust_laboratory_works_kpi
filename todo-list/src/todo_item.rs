use serde::{Deserialize, Serialize}; // для читання та створення json

use crate::ListItem;

// оголошення структури
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListItemData {
    pub completed: bool,
    pub description: String,
    pub datetime: String,
}

// конверсія для узгодженості типів даних
impl From<ListItemData> for ListItem {
    fn from(val: ListItemData) -> Self {
        ListItem {
            completed: val.completed,
            description: val.description.into(),
            datetime: val.datetime.into(),
        }
    }
}

impl From<ListItem> for ListItemData {
    fn from(value: ListItem) -> Self {
        Self {
            completed: value.completed,
            description: value.description.to_string(),
            datetime: value.datetime.to_string(),
        }
    }
}
