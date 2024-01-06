use itertools::Itertools;

use crate::{action::Action, category::Category, part::Part};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Condition {
    ordering: Ordering,
    category: Category,
    rhs: usize,
    action: Action,
}

impl Condition {
    pub fn get_action(&self, lhs: &Part) -> Option<Action> {
        if lhs.0.get(&self.category)?.cmp(&self.rhs) == self.ordering {
            Some(self.action.clone())
        } else {
            None
        }
    }
}

impl TryFrom<&str> for Condition {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (condition, send_to) = value.split(':').collect_tuple().ok_or(())?;
        let ordering = {
            if condition.find('<').is_some() {
                Ordering::Less
            } else if condition.find('>').is_some() {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        };
        let (category, rhs) = condition.split(['>', '<', '=']).collect_tuple().ok_or(())?;
        let category = Category::try_from(category.chars().next().ok_or(())?)?;
        let rhs = rhs.parse::<usize>().map_err(|_| ())?;
        let action = Action::from(send_to);
        Ok(Self {
            ordering,
            category,
            rhs,
            action,
        })
    }
}
