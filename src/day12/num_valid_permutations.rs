use cached::proc_macro::cached;

#[cached]
pub fn num_valid_permutations(record: String, groups: Vec<usize>) -> usize {
    if record.is_empty() {
        return if groups.is_empty() { 1 } else { 0 };
    }

    if groups.is_empty() {
        return if record.contains('#') { 0 } else { 1 };
    }

    let (c, rest_of_record) = record.split_at(1);
    match c {
        "." => num_valid_permutations(rest_of_record.to_string(), groups),
        "#" => {
            let group = groups[0];
            if record.len() >= group
                && record.chars().take(group).all(|c| c != '.')
                && (record.len() == group || record.get(group..group + 1) != Some("#"))
            {
                if record.len() > group {
                    num_valid_permutations(
                        record.split_at(group + 1).1.to_string(),
                        groups[1..].to_vec(),
                    )
                } else {
                    num_valid_permutations(String::default(), groups[1..].to_vec())
                }
            } else {
                0
            }
        }
        "?" => {
            num_valid_permutations(String::from("#") + rest_of_record, groups.clone())
                + num_valid_permutations(String::from(".") + rest_of_record, groups)
        }
        _ => unreachable!(),
    }
}
