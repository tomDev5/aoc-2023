use std::collections::HashMap;

use itertools::Itertools;

use crate::category::Category;

#[derive(Debug, Clone)]
pub struct Part(pub HashMap<Category, usize>);

impl TryFrom<&str> for Part {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            value[1..value.len() - 1]
                .split(',')
                .filter_map(|v| v.split('=').collect_tuple())
                .filter_map(|(category, value)| {
                    let category = Category::try_from(category.chars().next()?).ok()?;
                    let value = value.parse::<usize>().ok()?;
                    Some((category, value))
                })
                .collect(),
        ))
    }
}
