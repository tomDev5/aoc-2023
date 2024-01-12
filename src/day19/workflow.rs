use itertools::Itertools;

use crate::{action::Action, condition::Condition, part::Part};

#[derive(Debug, Clone)]
pub struct Workflow {
    pub conditions: Vec<Condition>,
    pub default: Action,
}

impl Workflow {
    pub fn get_action(&self, part: &Part) -> Action {
        for condition in &self.conditions {
            match condition.get_action(part) {
                Some(action) => return action,
                None => continue,
            }
        }
        self.default.clone()
    }
}

impl TryFrom<&str> for Workflow {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut conditions = value.split(',').collect_vec();
        let default = Action::from(conditions.pop().ok_or(())?);
        let conditions = conditions
            .into_iter()
            .filter_map(|condition| Condition::try_from(condition).ok())
            .collect_vec();
        Ok(Self {
            conditions,
            default,
        })
    }
}
